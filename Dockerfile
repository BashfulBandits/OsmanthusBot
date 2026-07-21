FROM nixos/nix

WORKDIR /workspace

ENV NIX_CONFIG="experimental-features = nix-command flakes"

COPY . .

#RUN nix develop --command true

#RUN rustup default stable

CMD ["nix", "develop", "--command", "cargo", "run"]
