{
  description = "Shell to be built into a Dockerfile";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in {
        devShells.default = pkgs.mkShell {
          packages = [ pkgs.cargo pkgs.rustc pkgs.yt-dlp ];

          buildInputs = [
            pkgs.openssl
            pkgs.pkg-config
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
