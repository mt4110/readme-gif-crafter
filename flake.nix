{
  description = "README GIF Crafter";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-26.05";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustVersion = (builtins.fromTOML (builtins.readFile ./.mise.toml)).tools.rust;
        rustToolchain = pkgs.rust-bin.stable.${rustVersion}.default.override {
          extensions = [ "rust-src" "rust-analyzer" "clippy" ];
        };
        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustToolchain;
          rustc = rustToolchain;
        };
        package = rustPlatform.buildRustPackage {
          pname = "readme-gif-crafter";
          version = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package.version;
          src = pkgs.lib.fileset.toSource {
            root = ./.;
            fileset = pkgs.lib.fileset.unions [ ./Cargo.toml ./Cargo.lock ./src ./tests ];
          };
          cargoLock.lockFile = ./Cargo.lock;
          nativeBuildInputs = [ pkgs.makeWrapper ];
          nativeCheckInputs = [ pkgs.ffmpeg ];
          postInstall = ''
            wrapProgram $out/bin/rgc --prefix PATH : ${pkgs.lib.makeBinPath [ pkgs.ffmpeg ]}
          '';
          meta.mainProgram = "rgc";
        };
      in {
        packages.default = package;
        checks = {
          build = package;
          runtime = pkgs.runCommand "rgc-runtime-check" { } ''
            ${pkgs.ffmpeg}/bin/ffmpeg -v error -f lavfi -i color=c=blue:s=32x32:d=0.2 input.mp4
            PATH= ${package}/bin/rgc input.mp4 --width 32 --fps 5
            ${pkgs.ffmpeg}/bin/ffmpeg -v error -i input.gif -f null -
            test -s input.gif
            touch $out
          '';
        };
        devShells.default = pkgs.mkShell {
          packages = [ rustToolchain pkgs.ffmpeg ];
        };
      });
}
