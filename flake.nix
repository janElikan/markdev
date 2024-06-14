{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    fenix.url = "github:nix-community/fenix";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, fenix, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };

        toolchain = with fenix.packages.${system}; combine [
          minimal.rustc
          minimal.cargo
          targets.wasm32-wasi.latest.rust-std
        ];

        naersk-lib = pkgs.callPackage naersk {
          cargo = toolchain;
          rustc = toolchain;
        };
      in
      {
        defaultPackage = naersk-lib.buildPackage {
            src = ./.;
            CARGO_BUILD_TARGET = "wasm32-wasi";
        };
        devShell = with pkgs; mkShell {
          buildInputs = [ toolchain rustfmt rustPackages.clippy bacon ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
          CARGO_BUILD_TARGET = "wasm32-wasi";
        };
      }
    );
}
