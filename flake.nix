{
  description = "cv flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay.url = "github:oxalica/rust-overlay";
    tailwind-src = {
      url = "https://github.com/tailwindlabs/tailwindcss/releases/download/v4.0.12/tailwindcss-linux-x64";
      flake = false;
    };
    rust-tools = {
      url = "github:pesukone/flakes?dir=rust";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
      tailwind-src,
      rust-tools,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
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
            rust-tools.packages.${system}.cargo-leptos
            packages.tailwindcss_4
            sass
          ];
        };
      }
    );
}
