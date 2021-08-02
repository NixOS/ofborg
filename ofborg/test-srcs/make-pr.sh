#!/usr/bin/env bash
set -eu

bare=$1
co=$2

export GIT_CONFIG_GLOBAL=/dev/null
export GIT_CONFIG_NOSYSTEM=1
export GIT_AUTHOR_NAME="GrahamCOfBorg"
export GIT_AUTHOR_EMAIL="graham+cofborg@example.com"

makepr() {
    git init --bare "$bare"
    git clone "$bare" "$co"

    cp build/* "$co/"
    git -C "$co" add .
    git -C "$co" commit -m "initial repo commit"
    git -C "$co" push origin master

    cp build-pr/*  "$co/"
    git -C "$co" checkout -b my-cool-pr
    git -C "$co" add .
    git -C "$co" commit -m "check out this cool PR"
    git -C "$co" push origin my-cool-pr:refs/pull/1/head
}

makepr >&2
git -C "$co" rev-parse HEAD
