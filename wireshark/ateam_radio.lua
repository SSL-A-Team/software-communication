-- ateam_radio.lua — Wireshark dissector for the A-Team radio link (proto format).
--
-- Wire format:  CRC32(4 bytes, little-endian) | varint(len) | RadioPacket bytes
--
-- The RadioPacket oneof field number is the packet discriminant; Wireshark reads
-- the .proto files directly so no field/enum names are hardcoded here.
--
-- ── Setup ───────────────────────────────────────────────────────────────────────
--
-- 1. Copy this file to your Wireshark personal Lua plugins directory:
--      Help → About Wireshark → Folders → Personal Lua Plugins
--    (Linux: ~/.local/lib/wireshark/plugins/)
--
-- 2. Add the proto search path in Wireshark:
--      Edit → Preferences → Protocols → Protobuf → Protobuf search paths
--    Add: <repo>/ateam-common-packets/proto/
--    Tick "Load all files in search paths on startup".
--
-- 3. Restart Wireshark.
--
-- 4. Set the UDP port if the radio link uses a fixed port:
--      Edit → Preferences → Protocols → A-Team Radio → UDP Port
--    (0 = heuristic detection only; right-click → Decode As to force a packet)
--
-- Requires Wireshark 3.4+ for the built-in protobuf dissector.
-- ────────────────────────────────────────────────────────────────────────────────

local ateam_radio = Proto("ateam_radio", "A-Team Radio")

-- ── Preferences ──────────────────────────────────────────────────────────────────

ateam_radio.prefs.udp_port = Pref.uint(
    "UDP Port", 0,
    "UDP port to decode as A-Team Radio (0 = heuristic detection only)"
)

-- ── Protocol fields ───────────────────────────────────────────────────────────────

local f_crc32   = ProtoField.uint32("ateam_radio.crc32",   "CRC32",          base.HEX)
local f_pb_len  = ProtoField.uint32("ateam_radio.pb_len",  "Payload Length", base.DEC)
local f_pb_data = ProtoField.bytes( "ateam_radio.pb_data", "RadioPacket (protobuf)")

ateam_radio.fields = { f_crc32, f_pb_len, f_pb_data }

-- ── Expert info ───────────────────────────────────────────────────────────────────

local ef_bad_varint = ProtoExpert.new(
    "ateam_radio.bad_varint", "Malformed varint length field",
    expert.group.MALFORMED, expert.severity.ERROR
)
local ef_truncated = ProtoExpert.new(
    "ateam_radio.truncated", "Packet truncated (length exceeds captured data)",
    expert.group.MALFORMED, expert.severity.WARN
)
local ef_proto_fail = ProtoExpert.new(
    "ateam_radio.proto_fail",
    "Protobuf decode failed — check proto search paths in Wireshark preferences",
    expert.group.UNDECODED, expert.severity.WARN
)

ateam_radio.experts = { ef_bad_varint, ef_truncated, ef_proto_fail }

-- ── Varint parser ─────────────────────────────────────────────────────────────────

-- Returns (value, next_offset) or (nil, offset) on error.
local function read_varint(tvb, offset)
    local value, shift, b = 0, 0
    local limit = math.min(offset + 5, tvb:len())  -- 32-bit varint: at most 5 bytes
    repeat
        if offset >= limit then return nil, offset end
        b = tvb(offset, 1):uint()
        offset = offset + 1
        value = value + bit32.lshift(bit32.band(b, 0x7F), shift)
        shift = shift + 7
    until bit32.band(b, 0x80) == 0
    return value, offset
end

-- ── Dissector ─────────────────────────────────────────────────────────────────────

function ateam_radio.dissector(tvb, pinfo, tree)
    local pkt_len = tvb:len()
    if pkt_len < 5 then return 0 end

    pinfo.cols.protocol:set("ATEAM")
    local subtree = tree:add(ateam_radio, tvb(), "A-Team Radio Packet")

    subtree:add_le(f_crc32, tvb(0, 4))

    local len, payload_off = read_varint(tvb, 4)
    if len == nil then
        subtree:add_proto_expert_info(ef_bad_varint)
        return pkt_len
    end

    local varint_bytes = payload_off - 4
    subtree:add(f_pb_len, tvb(4, varint_bytes))
        :set_text(string.format("Payload Length: %d bytes", len))

    if payload_off + len > pkt_len then
        subtree:add_proto_expert_info(ef_truncated)
        return pkt_len
    end

    local pb_item = subtree:add(f_pb_data, tvb(payload_off, len))
    pb_item:set_text(string.format("RadioPacket (%d bytes)", len))

    pinfo.private["pb_msg_name"] = "ateam.RadioPacket"
    local ok, err = pcall(function()
        Dissector.get("protobuf"):call(tvb(payload_off, len):tvb(), pinfo, pb_item)
    end)
    if not ok then
        pb_item:add_proto_expert_info(ef_proto_fail, tostring(err))
    end

    pinfo.cols.info:set(string.format("RadioPacket (%d bytes)", len))
    return pkt_len
end

-- ── Registration ─────────────────────────────────────────────────────────────────

local udp_table = DissectorTable.get("udp.port")

function ateam_radio.prefs_changed()
    pcall(function() udp_table:remove_all(ateam_radio) end)
    local port = ateam_radio.prefs.udp_port
    if port ~= 0 then udp_table:add(port, ateam_radio) end
end

ateam_radio.prefs_changed()

ateam_radio:register_heuristic("udp", function(tvb, pinfo, tree)
    if tvb:len() < 5 then return false end
    local len, payload_off = read_varint(tvb, 4)
    if len == nil then return false end
    if payload_off + len ~= tvb:len() then return false end
    ateam_radio.dissector(tvb, pinfo, tree)
    return true
end)
