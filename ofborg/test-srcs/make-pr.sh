#!/bin/sh

set -eux


(
    rm -rf bare-repo repo-co

    git init --bare bare-repo
    git clone ./bare-repo ./repo-co


    (
        cp build/* repo-co/
        cd repo-co
        git add .
        git commit --no-gpg-sign -m "initial repo commit"
        git push origin master
    )

    (
        cp build-pr/* repo-co/
        cd repo-co
        git checkout -b my-cool-pr
        git add .
        git commit --no-gpg-sign -m "check out this cool PR"
        git push origin my-cool-pr:refs/pull/1/head

    )
) >&2

(
    cd repo-co
    git rev-parse HEAD
)
