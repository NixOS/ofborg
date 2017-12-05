#!/bin/sh

cd nix

carnix ./../ofborg/Cargo.lock  --output ./ofborg-carnix.nix
