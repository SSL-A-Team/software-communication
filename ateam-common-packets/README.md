# Radio Packets

This folder includes all common packets sent via radio, which requires a C/Rust interface.

The headers are defined in C and generated for Rust using bind gen.

## Including C Code

You can include the C headers into any C/C++ program as you would normally.


## Building

Enter the development environment shell using the direction in the top level readme.

Run `cargo build` to generate the bindings, and verify they compile.

Run `cargo test` to run bindgen packing tests.


