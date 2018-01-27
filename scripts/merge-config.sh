#!/usr/bin/env nix-shell
#!nix-shell -p bash -p jq -p curl -i bash

jq -s '.[0] * .[1] * .[2]' ./config.public.json ./config.known-users.json ./config.private.json > ./config.prod.json
