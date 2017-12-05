#!/bin/sh

cd nix

carnix ./../ofborg/Cargo.lock  --output ./ofborg-carnix.nix
carnix ./../ircbot/Cargo.lock  --output ./ircbot-carnix.nix
