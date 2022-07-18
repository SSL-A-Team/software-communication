# Software Common Respository

Contains software artifacts shared across the firmware/software boundary.

Sub folders contain relevant sub README files.
 - radio/ - all common headers used in Robot\<-\>AI communications.

# Development Setup

These artifacts are generally included in other projects that produce actual
build artifacts. As such, there is no default setup. A nix flake is included
to independently support bindgen if desired. Nix setup is described in
the [firmware repository readme](https://github.com/SSL-A-Team/firmware/blob/main/README.md).

