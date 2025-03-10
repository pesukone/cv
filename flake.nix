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
    tailwind-src = {
      url = "https://github.com/tailwindlabs/tailwindcss/releases/download/v4.0.12/tailwindcss-linux-x64";
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
      tailwind-src,
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

        packages.tailwindcss_4 = pkgs.stdenv.mkDerivation {
          name = "tailwindcss";
          src = tailwind-src;

          nativeBuildInputs = with pkgs; [
            autoPatchelfHook
            makeWrapper
          ];

          dontUnpack = true;
          dontBuild = true;
          dontStrip = true;

          installPhase = ''
            mkdir -p $out/bin
            install -m755 $src $out/bin/tailwindcss
          '';

          postFixup = ''
            wrapProgram $out/bin/tailwindcss --prefix LD_LIBRARY_PATH : ${
              pkgs.lib.makeLibraryPath [ pkgs.stdenv.cc.cc.lib ]
            }
          '';
        };

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
            packages.tailwindcss_4
            sass
          ];
        };
      }
    );
}
