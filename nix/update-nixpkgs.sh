#!/bin/sh

nix-prefetch-git https://github.com/nixos/nixpkgs-channels.git \
                 --rev refs/heads/nixpkgs-unstable > ./nix/nixpkgs.json
