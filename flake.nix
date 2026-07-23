{
  description = "Shell to be built into a Dockerfile";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
      in {
        devShells.default = pkgs.mkShell {
          #packages = [ ];

          buildInputs = [
            pkgs.openssl
            pkgs.pkg-config
            pkgs.yt-dlp
            pkgs.rust-bin.stable.latest.default
          ];

          nativeBuildInputs = [
            pkgs.cmake
          ];

          shellHook = ''
            if [ -f .env ]; then
            set -a
            source .env
            set +a
            fi


            export LD_LIBRARY_PATH=${pkgs.openssl}/lib:$LD_LIBRARY_PATH
            export LD_LIBRARY_PATH=${pkgs.pkg-config}/lib:$LD_LIBRARY_PATH
          '';
        };
      });
}
