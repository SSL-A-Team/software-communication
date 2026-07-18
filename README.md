# Software Common Repository ![Build Status Badge](https://github.com/SSL-A-Team/common/actions/workflows/CI.yml/badge.svg)

Shared artifacts across the firmware/software boundary: packet definitions, proto schemas, and generated bindings.

## Repository Structure

```
ateam-common-packets/   Robot↔AI communication packet definitions (C headers, protos, Rust bindings)
ssl-league-protobufs/   SSL league proto definitions (game controller, vision, simulation)
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

## Sub-package READMEs

- [`ateam-common-packets/README.md`](ateam-common-packets/README.md) — C headers, proto schemas, ROS2 msg generation, Rust bindings
