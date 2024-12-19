{
  # inputs = {
  #   nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  #   flake-utils.url = "github:numtide/flake-utils";

  #   crane.url = "github:ipetkov/crane";
  #   rust-overlay = {
  #     url = "github:oxalica/rust-overlay";
  #     inputs.nixpkgs.follows = "nixpkgs";
  #   };
  # };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
      crane,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        src = ./.;
        buildInputs = [ ];
        nativeBuildInputs = with pkgs; [
          clang_15
          mold
          rustToolchain
        ];

        commonArgs = {
          pname = "cohle";
          version = "latest";
          strictDeps = true;
          LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
          stdenv = pkgs.stdenvAdapters.useMoldLinker pkgs.llvmPackages_15.stdenv;
          CARGO_BUILD_RUSTFLAGS = "-C linker=clang -C link-arg=-fuse-ld=${pkgs.mold}/bin/mold";
          inherit src buildInputs nativeBuildInputs;
        };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
        bin = craneLib.buildPackage (
          commonArgs
          // {
            inherit cargoArtifacts;
          }
        );
      in
      with pkgs;
      {
        checks = {
          inherit bin;

          told-clippy = craneLib.cargoClippy (
            commonArgs
            // {
              inherit cargoArtifacts;
            }
          );
        };

        packages = {
          inherit bin;
          default = bin;
          cohle = bin;
        };

        devShells.default = mkShell {
          inputsFrom = [ bin ];
          LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
          CARGO_BUILD_RUSTFLAGS = "-C linker=clang -C link-arg=-fuse-ld=${pkgs.mold}/bin/mold";
          shellHook = "zsh";
        };
      }
    );
}
