#!/usr/bin/env nix-shell
#!nix-shell -i bash -E "with import <nixpkgs> {}; mkShell { nativeBuildInputs = [ (import (builtins.fetchTarball https://github.com/kolloch/crate2nix/archive/0.8.0.tar.gz) {}) ]; }"
set -e

cargo fetch --locked
crate2nix generate
