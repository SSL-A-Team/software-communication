{
  description = "The common software repository for the SSL A-Team.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
    # uv2nix: build Python env from uv.lock (run `uv lock` after editing pyproject.toml)
    uv2nix.url = "github:pyproject-nix/uv2nix";
    uv2nix.inputs.nixpkgs.follows = "nixpkgs";
    pyproject-nix.url = "github:pyproject-nix/pyproject.nix";
    pyproject-nix.inputs.nixpkgs.follows = "nixpkgs";
    pyproject-build-systems.url = "github:pyproject-nix/build-system-pkgs";
    pyproject-build-systems.inputs.pyproject-nix.follows = "pyproject-nix";
    pyproject-build-systems.inputs.uv2nix.follows = "uv2nix";
    pyproject-build-systems.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, uv2nix, pyproject-nix, pyproject-build-systems, ... }:
    flake-utils.lib.eachSystem [
        "aarch64-linux"
        "aarch64-darwin"
        "x86_64-darwin"
        "x86_64-linux" ]
    (system:
      let
        inherit (nixpkgs) lib;

        overlays = [ (import rust-overlay) ];

        pkgs = import nixpkgs {
          inherit system overlays;
        };

        packageName = "ateam-firmware";

        # uv2nix: Python env derived from pyproject.toml + uv.lock.
        # Run `uv lock` after editing pyproject.toml to regenerate uv.lock.
        workspace = uv2nix.lib.workspace.loadWorkspace { workspaceRoot = ./.; };

        # Third-party packages resolved from uv.lock + build-system backends.
        pythonOverlay = workspace.mkPyprojectOverlay { sourcePreference = "wheel"; };

        # pythonSet: all packages from uv.lock + build-system backends (hatchling etc).
        # The root project (ateam-common-packets-tools) is virtual (tool.uv.package=false)
        # so uv2nix skips building it and only installs its declared dependencies.
        pythonSet = (pkgs.callPackage pyproject-nix.build.packages {
          python = pkgs.python3;
        }).overrideScope (lib.composeManyExtensions [
          pyproject-build-systems.overlays.default
          pythonOverlay
        ]);

        pythonEnv = pythonSet.mkVirtualEnv "ateam-tools-env" workspace.deps.default;

      in {
        devShell = pkgs.mkShell {
          shellHook = ''
          export LIBCLANG_PATH="${pkgs.libclang.lib}/lib"
          '';

          buildInputs = with pkgs; [
            gnumake

            # GCC ARM Embedded 14 provides the sysroot/ABI defining types and type sizes
            # for bindgen
            gcc-arm-embedded-14

            # needed by bindgen
            clang

            # needed by micropb-gen (build-time proto compiler)
            protobuf

            # Python env from uv.lock (protobuf + pytest for protoc_gen_ros2msg.py)
            pythonEnv
            uv

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
