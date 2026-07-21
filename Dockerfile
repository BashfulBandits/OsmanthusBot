FROM nixos/nix

WORKDIR /workspace

ENV NIX_CONFIG="experimental-features = nix-command flakes"

COPY . .

CMD ["nix", "develop", "--command", "cargo", "run"]
