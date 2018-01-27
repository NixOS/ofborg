#!/usr/bin/env nix-shell
#!nix-shell -p bash -p jq -p curl -i bash

readonly token=$(jq -r '.github.token' ./config.private.json)

readonly dest=config.known-users.json
readonly scratch=user-list.scratch
readonly accumulator=user-list.accumulator
readonly result=user-list.result

function fetch_users() {
    curl \
        -H "Authorization: token $token" \
        "https://api.github.com/orgs/NixOS/members?page=$1" \
        | jq 'map(.login | ascii_downcase)'
}

echo '[]' > "$accumulator"

page=0
while true; do
    page=$((page + 1))
    fetch_users "$page" > "$scratch"

    jq -s '.[0] + .[1]' "$accumulator" "$scratch" > "$result"
    mv "$result" "$accumulator"

    if [ $(jq -r 'length' "$scratch") -eq 0 ]; then
        break
    fi
done

jq -s '{ "runner": { "known_users": .[0]}}' "$accumulator" > "$dest"

rm -f "$result" "$scratch" "$accumulator"
