{
  description = "The common software repository for the SSL A-Team.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachSystem [
        "aarch64-linux"
        "aarch64-darwin"
        "x86_64-darwin"
        "x86_64-linux" ]
    (system: 
      let 
        overlays = [ (import rust-overlay) ];

        pkgs = import nixpkgs {
          inherit system overlays;
        };

        packageName = "ateam-firmware";

      in {
        devShell = pkgs.mkShell {
          # needed by bindgen to invoke clang
          shellHook = ''
          export LIBCLANG_PATH="${pkgs.libclang.lib}/lib"
          '';

          buildInputs = with pkgs; [
            gnumake

            # GCC ARM Embedded 12 provides the sysroot/ABI defining types and type sizes
            # for bindgen
            gcc-arm-embedded-12

            # needed by bindgen
            clang

            # Rust Embedded
            (rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
              extensions = [ "rust-src" ];
              targets = [ "thumbv7em-none-eabihf" "thumbv6m-none-eabi" ];
            }))
            rust-analyzer
          ];

        };
      }
    );
}
