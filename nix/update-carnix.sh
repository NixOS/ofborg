#!/bin/sh

set -eu

cd ofborg

cargo build
carnix generate-nix --src .
