#!/bin/sh

set -eu

cd nix

patched_carnix() {
    src=$1
    result=$2

    (
        cd "$(dirname "$src")"
        cargo build
    )

    carnix "$src"  --output "$result"
    patch -p1 "$result" ./carnix.patch
}

patched_carnix ./../ofborg/Cargo.lock ./ofborg-carnix.nix
