{
  description = "cv flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay
  }:
    flake-utils.lib.eachDefaultSystem (system:
      let 
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { 
	  inherit system overlays;
	};

      in rec {
        flakedPkgs = pkgs;

	devShell = pkgs.mkShell {
	  buildInputs = with pkgs; [
	    (rust-bin.selectLatestNightlyWith (toolchain:
	      toolchain.default.override {
	        extensions = [ "rust-src" ];
		targets = [ "wasm32-unknown-unknown" ];
	      }
	    ))
	    cargo-leptos
	    sass
	    lld
	  ];
	};
      }
    );
}
