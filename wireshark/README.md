# Wireshark Dissector

`ateam_radio.lua` decodes A-Team radio link traffic in Wireshark.

**Wire format:** `CRC32 (4 bytes, little-endian) | varint(len) | RadioPacket bytes`

The `RadioPacket` protobuf oneof field number is the packet discriminant. Wireshark reads the `.proto` files at runtime — no field names, enum values, or message types are hardcoded in the Lua.

## Installation

```sh
make install-wireshark-plugin      # auto-detects plugin directory
make uninstall-wireshark-plugin    # removes it
```

If `make` cannot detect the path (Wireshark not on `$PATH`), copy `ateam_radio.lua` manually to the directory shown under **Help → About Wireshark → Folders → Personal Lua Plugins**.

After installing, reload without restarting: **Ctrl+Shift+L** (Analyze → Reload Lua Plugins).

## Proto file setup

For full field-name decoding, point Wireshark at the proto schemas:

1. **Edit → Preferences → Protocols → Protobuf → Protobuf search paths**
2. Add `<repo>/ateam-common-packets/proto/`
3. Tick **Load all files in search paths on startup**
4. Restart Wireshark (required after changing proto paths)

Without this step the dissector still shows the CRC32, length, and raw payload bytes, but field names and enum values will be missing.

## Port configuration

The dissector registers a UDP heuristic: any UDP packet whose byte layout matches the wire format is automatically decoded.

To force-decode a specific port:

- **Edit → Preferences → Protocols → A-Team Radio → UDP Port** — set to your radio link port
- Or right-click any packet → **Decode As → A-Team Radio**

## Requires

Wireshark 3.4 or later (built-in protobuf dissector).
