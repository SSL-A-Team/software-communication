# Software Common Repository ![Build Status Badge](https://github.com/SSL-A-Team/common/actions/workflows/CI.yml/badge.svg)

Shared artifacts across the firmware/software boundary: packet definitions, proto schemas, and generated bindings.

## Repository Structure

```
ateam-common-packets/   Robot↔AI communication packet definitions (C headers, protos, Rust bindings)
ssl-league-protobufs/   SSL league proto definitions (game controller, vision, simulation)
wireshark/              Wireshark Lua dissector for the radio link
flake.nix               Nix dev environment (protoc, Python, arm-none-eabi-gcc for bindgen)
Makefile                Top-level build/test targets
```

## Development Setup

A Nix flake provides all required tools. See the [firmware repository README](https://github.com/SSL-A-Team/firmware/blob/main/README.md) for Nix setup instructions.

```sh
nix develop       # enter dev shell (protoc, Python 3, arm-none-eabi-gcc, cargo)
make test         # run all test suites
make              # build Rust bindings
```

## Wireshark Dissector

[`wireshark/`](wireshark/README.md) — Lua dissector for the radio link. Decodes `CRC32 | varint(len) | RadioPacket` frames; delegates field decoding to Wireshark's built-in protobuf dissector using the `.proto` files in this repo.

```sh
make install-wireshark-plugin    # install to Wireshark personal plugins directory
make uninstall-wireshark-plugin  # remove it
```

## Sub-package READMEs

- [`ateam-common-packets/README.md`](ateam-common-packets/README.md) — C headers, proto schemas, ROS2 msg generation, Rust bindings
