-- ateam_radio.lua — Wireshark dissector for the A-Team radio link.
--
-- Supports two wire formats:
--
--   Proto (new):   CRC32(4 LE) | varint(len) | RadioPacket bytes
--   Legacy C:      CRC32(4 LE) | CommandCode(1) | reserved(1) | data_length(2 LE) | data
--
-- Auto-detection heuristic: if byte[4] is a known CommandCode value AND byte[5]
-- is 0x00 (reserved), the packet is treated as Legacy C.  Otherwise proto is tried.
-- Override via the "Packet format" preference.
--
-- ── Setup ───────────────────────────────────────────────────────────────────────
--
-- 1. Copy this file to your Wireshark personal Lua plugins directory:
--      Help → About Wireshark → Folders → Personal Lua Plugins
--    (on Linux: ~/.local/lib/wireshark/plugins/)
--
-- 2. For proto format decoding, point Wireshark at the .proto files:
--      Edit → Preferences → Protocols → Protobuf → Protobuf search paths
--    Add the path to ateam-common-packets/proto/ (and ssl-league-protobufs/proto/
--    if your capture includes SSL traffic).
--    Tick "Load all files in search paths on startup".
--
-- 3. Restart Wireshark.
--
-- 4. If your radio link uses a fixed UDP port, set it in:
--      Edit → Preferences → Protocols → A-Team Radio → UDP Port
--    Set to 0 to rely on heuristic detection only (right-click a packet and
--    choose "Decode As → A-Team Radio" to force-decode individual packets).
--
-- ── Requires ────────────────────────────────────────────────────────────────────
--   Wireshark 3.4+ (built-in protobuf dissector; earlier versions will dissect
--   the framing layer but leave the payload as raw bytes).
-- ────────────────────────────────────────────────────────────────────────────────

local ateam_radio = Proto("ateam_radio", "A-Team Radio")

-- ── Preferences ─────────────────────────────────────────────────────────────────

local FORMAT_AUTO   = 0
local FORMAT_PROTO  = 1
local FORMAT_LEGACY = 2

ateam_radio.prefs.udp_port = Pref.uint(
    "UDP Port", 0,
    "UDP port to decode as A-Team Radio (0 = heuristic detection only)"
)
ateam_radio.prefs.format = Pref.enum(
    "Packet format", FORMAT_AUTO, "Wire format selection",
    {{"Auto-detect", FORMAT_AUTO}, {"Proto (new)", FORMAT_PROTO}, {"Legacy C struct", FORMAT_LEGACY}},
    false
)

-- ── Protocol fields ──────────────────────────────────────────────────────────────

local CC_NAMES = {
    [1]  = "CC_ACK",
    [2]  = "CC_NACK",
    [3]  = "CC_GOODBYE",
    [4]  = "CC_KEEPALIVE",
    [21] = "CC_HELLO_REQ",
    [22] = "CC_HELLO_RESP",
    [41] = "CC_TELEMETRY",
    [42] = "CC_CONTROL_DEBUG_TELEMETRY",
    [43] = "CC_ROBOT_PARAMETER_COMMAND",
    [44] = "CC_ERROR_TELEMETRY",
    [61] = "CC_CONTROL",
}

local CC_VALID = {}
for k in pairs(CC_NAMES) do CC_VALID[k] = true end

-- Shared
local f_crc32    = ProtoField.uint32("ateam_radio.crc32",    "CRC32",    base.HEX)
local f_format   = ProtoField.string("ateam_radio.format",   "Detected Format")

-- Proto format
local f_pb_len   = ProtoField.uint32("ateam_radio.pb_len",   "Payload Length (varint)", base.DEC)
local f_pb_data  = ProtoField.bytes( "ateam_radio.pb_data",  "RadioPacket (protobuf)")

-- Legacy format
local f_cc       = ProtoField.uint8( "ateam_radio.cc",       "Command Code", base.HEX, CC_NAMES)
local f_reserved = ProtoField.uint8( "ateam_radio.reserved", "Reserved",     base.HEX)
local f_data_len = ProtoField.uint16("ateam_radio.data_len", "Data Length",  base.DEC)
local f_data     = ProtoField.bytes( "ateam_radio.data",     "Data")

ateam_radio.fields = {
    f_crc32, f_format,
    f_pb_len, f_pb_data,
    f_cc, f_reserved, f_data_len, f_data,
}

-- ── Varint parser ────────────────────────────────────────────────────────────────

-- Read a protobuf base-128 varint from tvb starting at byte offset.
-- Returns (value, next_offset) or (nil, offset) on error.
local function read_varint(tvb, offset)
    local value = 0
    local shift = 0
    local b
    local limit = math.min(offset + 5, tvb:len())  -- max 5 bytes for a 32-bit varint
    repeat
        if offset >= limit then return nil, offset end
        b = tvb(offset, 1):uint()
        offset = offset + 1
        value = value + bit32.lshift(bit32.band(b, 0x7F), shift)
        shift = shift + 7
    until bit32.band(b, 0x80) == 0
    return value, offset
end

-- ── Format detection ─────────────────────────────────────────────────────────────

-- Returns FORMAT_PROTO or FORMAT_LEGACY or nil (unknown / too short).
local function detect_format(tvb)
    if tvb:len() < 8 then return nil end
    local b4 = tvb(4, 1):uint()
    local b5 = tvb(5, 1):uint()
    -- Legacy heuristic: byte[4] is a known CommandCode AND byte[5] is the reserved 0x00.
    if CC_VALID[b4] and b5 == 0x00 then
        -- Double-check: data_length in bytes[6:7] should not exceed remaining packet.
        local data_len = tvb(6, 2):le_uint()
        if 8 + data_len <= tvb:len() then
            return FORMAT_LEGACY
        end
    end
    -- Proto heuristic: varint at byte[4] gives a length that exactly spans the packet.
    local len, next_off = read_varint(tvb, 4)
    if len ~= nil and (next_off + len) == tvb:len() then
        return FORMAT_PROTO
    end
    return nil
end

-- ── Main dissector ───────────────────────────────────────────────────────────────

function ateam_radio.dissector(tvb, pinfo, tree)
    local pkt_len = tvb:len()
    if pkt_len < 5 then return 0 end

    pinfo.cols.protocol:set("ATEAM")

    local subtree = tree:add(ateam_radio, tvb(), "A-Team Radio Packet")

    -- CRC32 is always the first 4 bytes (little-endian on ARM).
    subtree:add_le(f_crc32, tvb(0, 4))

    -- Resolve format.
    local fmt = ateam_radio.prefs.format
    local detected_label
    if fmt == FORMAT_AUTO then
        fmt = detect_format(tvb)
        if fmt == FORMAT_PROTO then
            detected_label = "proto (auto)"
        elseif fmt == FORMAT_LEGACY then
            detected_label = "legacy C struct (auto)"
        else
            detected_label = "unknown"
            fmt = FORMAT_PROTO  -- best-effort fallback
        end
    elseif fmt == FORMAT_PROTO then
        detected_label = "proto (forced)"
    else
        detected_label = "legacy C struct (forced)"
    end
    subtree:add(f_format, detected_label)

    if fmt == FORMAT_PROTO then
        -- ── Proto format: varint(len) | RadioPacket bytes ──
        local len, payload_off = read_varint(tvb, 4)
        if len == nil then
            subtree:add_proto_expert_info(ateam_radio.experts.malformed_varint)
            return pkt_len
        end

        local varint_len = payload_off - 4
        local len_item = subtree:add(f_pb_len, tvb(4, varint_len))
        len_item:set_text(string.format("Payload Length: %d bytes (%d-byte varint)", len, varint_len))

        if payload_off + len > pkt_len then
            subtree:add_proto_expert_info(ateam_radio.experts.truncated)
            return pkt_len
        end

        local payload_tvb = tvb(payload_off, len):tvb()
        local pb_item = subtree:add(f_pb_data, tvb(payload_off, len))
        pb_item:set_text(string.format("RadioPacket (%d bytes)", len))

        -- Delegate to Wireshark's built-in protobuf dissector.
        -- pinfo.private["pb_msg_name"] tells it which message type to use.
        pinfo.private["pb_msg_name"] = "ateam.RadioPacket"
        local ok, err = pcall(function()
            Dissector.get("protobuf"):call(payload_tvb, pinfo, pb_item)
        end)
        if not ok then
            pb_item:add_proto_expert_info(ateam_radio.experts.proto_decode_failed,
                tostring(err))
        end

        pinfo.cols.info:set(string.format("RadioPacket [proto, %d bytes]", len))

    else
        -- ── Legacy C struct format ──
        -- RadioHeader: CommandCode(1) | reserved(1) | data_length(2 LE) | data
        if pkt_len < 8 then
            subtree:add_proto_expert_info(ateam_radio.experts.truncated)
            return pkt_len
        end

        local cc    = tvb(4, 1):uint()
        local cc_name = CC_NAMES[cc] or string.format("0x%02x (unknown)", cc)

        subtree:add(f_cc,       tvb(4, 1))
        subtree:add(f_reserved, tvb(5, 1))
        subtree:add_le(f_data_len, tvb(6, 2))

        local data_len = tvb(6, 2):le_uint()
        if pkt_len >= 8 + data_len and data_len > 0 then
            subtree:add(f_data, tvb(8, data_len))
        elseif data_len > 0 then
            subtree:add_proto_expert_info(ateam_radio.experts.truncated)
        end

        pinfo.cols.info:set(string.format("%s [legacy, %d bytes]", cc_name, data_len))
    end

    return pkt_len
end

-- ── Expert info ──────────────────────────────────────────────────────────────────

local ef_malformed = ProtoExpert.new(
    "ateam_radio.malformed_varint", "Malformed varint length field",
    expert.group.MALFORMED, expert.severity.ERROR
)
local ef_truncated = ProtoExpert.new(
    "ateam_radio.truncated", "Packet truncated (length exceeds captured data)",
    expert.group.MALFORMED, expert.severity.WARN
)
local ef_proto_fail = ProtoExpert.new(
    "ateam_radio.proto_decode_failed", "Protobuf decode failed (check proto search paths in preferences)",
    expert.group.UNDECODED, expert.severity.WARN
)

ateam_radio.experts = {
    malformed_varint   = ef_malformed,
    truncated          = ef_truncated,
    proto_decode_failed = ef_proto_fail,
}

-- ── Port table registration ───────────────────────────────────────────────────────

local udp_table = DissectorTable.get("udp.port")

function ateam_radio.prefs_changed()
    -- Re-register whenever the UDP port preference changes.
    -- Remove all previous registrations first (Wireshark 4.x API).
    pcall(function() udp_table:remove_all(ateam_radio) end)
    local port = ateam_radio.prefs.udp_port
    if port ~= 0 then
        udp_table:add(port, ateam_radio)
    end
end

-- Apply initial port at load time.
ateam_radio.prefs_changed()

-- ── Heuristic registration ────────────────────────────────────────────────────────

local function heuristic_check(tvb, pinfo, tree)
    if tvb:len() < 8 then return false end
    local fmt = detect_format(tvb)
    if fmt == nil then return false end
    ateam_radio.dissector(tvb, pinfo, tree)
    return true
end

ateam_radio:register_heuristic("udp", heuristic_check)
