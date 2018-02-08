#!/bin/sh

cd nix

patched_carnix() {
    src=$1
    result=$2

    carnix "$src"  --output "$result"
    patch -p1 "$result" ./carnix.patch
}

patched_carnix ./../ofborg/Cargo.lock ./ofborg-carnix.nix
patched_carnix ./../ircbot/Cargo.lock ./ircbot-carnix.nix
