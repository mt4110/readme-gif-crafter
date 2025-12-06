{
  description = "README GIF Crafter";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        
        # Exact version pinning matching .mise.toml
        # We need 1.82.0 or newer. Let's use 1.82.0.
        rustToolchain = pkgs.rust-bin.stable."1.82.0".default.override {
            extensions = [ "rust-src" "rust-analyzer" "clippy" ];
        };
        
      in {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "readme-gif-crafter";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          nativeBuildInputs = [ pkgs.pkg-config ];
          buildInputs = [ pkgs.ffmpeg ];
          # For buildRustPackage we might need to point to the correct rustc/cargo if it differs from stdenv
          # But for simple devShell usage below is key.
        };

        devShells.default = pkgs.mkShell {
          buildInputs = [
            rustToolchain
            pkgs.ffmpeg
            # pkgs.rustfmt # Included in rustToolchain
            # pkgs.clippy # Included in rustToolchain
          ];
          
          # Hook to ensure mise and nix play nice? 
          # User asked for "Mise and flake.nix to coexist well".
          # Usually if we have rustToolchain here, it puts it in PATH.
          # If user has mise hooks, mise might override it or vice versa.
          # Best practice: Provide the env here for those who want pure nix.
          # Those who want mise can just use `nix develop --command bash` but use mise's rust if setup.
          # BUT, since we pinned 1.82.0 here, it matches mise. So no conflict!
        };
      });
}
