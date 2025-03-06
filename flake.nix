{
  description = "cv flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    cargo-leptos-src = {
      url = "github:leptos-rs/cargo-leptos";
      flake = false;
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
      cargo-leptos-src,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [
          (import rust-overlay)
          (final: prev: {
            cargo-leptos = prev.rustPlatform.buildRustPackage {
              pname = "cargo-leptos";
              version = "0.2.29";

              src = cargo-leptos-src;
              useFetchCargoVendor = true;
              cargoHash = "sha256-KlB4/1cqFF59xzSqQBzqWRiPoClw/uSk4Y9ZJYbm8/M=";

              buildInputs = with prev.pkgs; [
                openssl.dev
              ];
              nativeBuildInputs = with prev.pkgs; [
                pkg-config
              ];

              buildFeatures = [ "no_downloads" ];
              doCheck = false;
            };
          })
        ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

      in
      rec {
        flakedPkgs = pkgs;

        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            (pkgs.rust-bin.selectLatestNightlyWith (
              toolchain:
              toolchain.default.override {
                extensions = [ "rust-src" ];
                targets = [ "wasm32-unknown-unknown" ];
              }
            ))
            cargo-leptos
            tailwindcss_4
            sass
          ];
        };
      }
    );
}
