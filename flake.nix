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
          LIBCLANG_PATH = "${pkgs.llvmPackages.libclang}/lib/libclang.so";

          shellHook = ''
          export LIBCLANG_PATH="${pkgs.libclang.lib}/lib"
          '';

          buildInputs = with pkgs; [
            gnumake
            clang

            # Rust Embedded
            rust-bin.stable.latest.default
          ];
        };
      }
    );
}
