#!/bin/sh
set -eu

bare=$1
co=$2

(
    git init --bare "$bare"
    git clone "$bare" "$co"


    (
        cp build/* "$co/"
        cd  "$co/"
        git add .
        git commit --no-gpg-sign --author "GrahamCOfBorg <graham+cofborg@example.com>" -m "initial repo commit"
        git push origin master
    )

    (
        cp build-pr/*  "$co/"
        cd "$co/"
        git checkout -b my-cool-pr
        git add .
        git commit --no-gpg-sign --author "GrahamCOfBorg <graham+cofborg@example.com>" -m "check out this cool PR"
        git push origin my-cool-pr:refs/pull/1/head

    )
) >&2

(
    cd  "$co/"
    git rev-parse HEAD
)
