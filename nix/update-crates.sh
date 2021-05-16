#!/usr/bin/env nix-shell
#!nix-shell -i bash -E "with import ./nix {}; mkShell { nativeBuildInputs = [ cacert cargo (import (builtins.fetchTarball https://github.com/kolloch/crate2nix/archive/0.9.0.tar.gz) {}) ]; }"
set -e

cargo fetch --locked
crate2nix generate
