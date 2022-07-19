# Software Common Respository ![Build Status Badge](https://github.com/SSL-A-Team/common/actions/workflows/CI.yml/badge.svg)

Contains software artifacts shared across the firmware/software boundary.

Sub folders contain relevant sub README files.
 - `ateam-common-packets/` - all packet definitions used in Robot\<-\>AI communications.
 - `radio-protocol/` - the top level radio communication spec (bot discovery, coms, etc)

# Development Setup

These artifacts are generally included in other projects that produce actual
build artifacts. As such, there is no default setup. A nix flake is included
to independently support bindgen if desired. Nix setup is described in
the [firmware repository readme](https://github.com/SSL-A-Team/firmware/blob/main/README.md).

