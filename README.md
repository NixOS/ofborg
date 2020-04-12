# grahamcofborg

## Guidelines

1. make sure you've reviewed the code before you trigger it on a PR
   that isn't your own
2. be gentle, preferably don't run mass rebuilds / massive builds like
   chromium on it

## Automatic Building

Users who are _trusted_ or _known_ (see: Trusted Users vs Known Users)
will have their PRs automatically trigger builds if their commits
follow the well-defined format of Nixpkgs. Specifically: prefixing the
commit title with the package attribute. This includes package bumps
as well as other changes.

Example messages and the builds:

| Message                                                               | Automatic Build                                          |
|-----------------------------------------------------------------------|----------------------------------------------------------|
| `vim: 1.0.0 -> 2.0.0`                                                 | `vim`                                                    |
| `vagrant: Fix dependencies for version 2.0.2 `                        | `vagrant`                                                |
| `python36Packages.requests,python27Packages.requests: 1.0.0 -> 2.0.0` | `python36Packages.requests`, `python27Packages.requests` |
| `python{2,3}Packages.requests: 1.0.0 -> 2.0.0`                        | _nothing_                                                |

If a PR is opened with many commits, it will create a single build job
for all of the detected packages. If a PR is opened and many commits
are pushed one by one to the open PR, many build jobs will be created.

To disable automatic building of packages on a PR, add `[WIP]` to the
PR's title, or the `2.status: work-in-progress` label.

## Commands

The comment parser is line-based, so comments can be interleaved with
instructions.

1. To trigger the bot, the line _must_ start with a case
   insensitive version of `@GrahamcOfBorg`.
2. To use multiple commands, insert a bit of whitespace and then your
   new command.

Commands:

### test (added: 2017-11-24)

```
@grahamcofborg test list of tests
```

This will run `nix-build ./nixos/release.nix -A tests.list -A tests.of -A tests.tests` in
the nixpkgs checkout. Note: this will only run on x86_64-linux machines.

### eval

```
@grahamcofborg eval
```

Note: Every PR automatically evaluates when it is opened and when the
commits change. There is no reason to run eval on a PR unless the
evaluation has failed for weird reasons, or because master was broken
before.

### build

```
@grahamcofborg build list of attrs
```

This will run `nix-build ./default.nix -A list -A of -A attrs` in
the nixpkgs checkout.

---


Multiple Commands:

```
@grahamcofborg build list of attrs
@grahamcofborg eval
```

or even:

```
@grahamcofborg build list of attrs @grahamcofborg eval
```

This will also work:

```
looks good to me!
@grahamcofborg build list of attrs
```

And this is fine:

```
@grahamcofborg build list of attrs
looks good to me!
```

This is will build `list`, `of`, `attrs`, `looks`, `good`, `to`, `me!`:

```
@grahamcofborg build list of attrs looks good to me!
```

## Trusted Users vs Known Users

Known users have their builds executed on platforms with working
sandboxing. At the time of writing, that means:

 - `x86_64-linux`
 - `aarch64_linux`

Trusted users have their builds executed on _all_ platforms, even if
they don't have good sandboxing. This opens the host up to a higher
risk of security issues, so only well known, trusted member of the
community should be added to the trusted user list.

At the time of writing, trusted users have their builds run on the
following platforms:

 - `x86_64-linux`
 - `aarch64_linux`
 - `x86_64-darwin`

See ./config.public.json and ./config.known-users.json for a list of
all the trusted and known users.

# How does OfBorg call nix-build?

Builds are run like:

> HOME=/homeless-shelter NIX_PATH=nixpkgs=$(pwd) nix-build ./default.nix
> --no-out-link --keep-going -A hello
> --option restrict-eval true
> --option build-timeout 1800
> --argstr system thesystem
> --show-trace

# How does OfBorg call nix-instantiate?

NixOS evals are run like:

> HOME=/homeless-shelter NIX_PATH=nixpkgs=$(pwd) nix-instantiate ./nixos/release.nix
> -A manual
> --option restrict-eval true
> --option build-timeout 1800
> --argstr system thesystem
> --show-trace

Nixpkgs evals are run like:

> HOME=/homeless-shelter NIX_PATH=nixpkgs=$(pwd) nix-instantiate ./pkgs/top-level/release.nix
> -A manual
> --option restrict-eval true
> --option build-timeout 1800
> --argstr system thesystem
> --show-trace


# Running meta checks locally

```
$ curl -o outpaths.nix https://raw.githubusercontent.com/NixOS/ofborg/released/ofborg/src/outpaths.nix
$ GC_INITIAL_HEAP_SIZE=4g nix-env -f ./outpaths.nix -qaP --no-name --out-path --arg checkMeta true > out-paths
```

---




# Running a builder

It is recommended to create a special user for the ofBorg operation.
This user should have git configuration for default username/email for
non-interactive merges. For example:
```
git config --global user.email "graham+cofborg@example.com"
git config --global user.name "GrahamCOfBorg"
```


```
nix-shell ./shell.nix
$ cd ofborg
$ cargo build
```

```
cargo build
```

then copy example.config.json to config.json and edit its vars. Set
`nix.remote` to an empty string if you're not using the daemon.

Run

```
./target/debug/builder ./config.json
```


Note the config.public.json for the public pieces of how I run ofborg,
which is merged with config.known-users.json and a third private
config file of credentials. These files contain some special keys like

 - known users
 - authorized users
 - log storage

they are only used in the backend processing tasks, and there is no
need for them on builders. However, to update the list in
config.known-users.json, run `./scripts/update-known-users.sh`.

In case you have a non-trivial setup on Linux, make sure that the ofborg
user has access to `dev/kvm`, as it is needed for running tests.

If you want to run multiple builder instances on the same physical
machine please make sure they use different configs with different
instance identity (same username/password is OK) and different
repository paths. Running two builders with the same config risks data
corruption.

See also: https://github.com/NixOS/ofborg/wiki/Operating-a-Builder

## Hacking

```
git clone https://github.com/NixOS/ofborg/
cd ofborg
nix-shell ./shell.nix
cd ofborg # enter subdirectory with Rust code
cargo build
cargo check
cargo test
```

Be sure to test your changes by running the following commands from the root of the repository:

```
nix-shell --run checkPhase -A mozilla-rust-overlay # checks rustfmt and Clippy
nix-shell --run checkPhase # runs actual tests
```

Currently there's no way to set-up a testing instance easily.
Send a PR if `check` and `test` are passing. Make sure
to format your code with `cargo fmt` and check for additional
warnings with `cargo clippy`.

To disable warnings as errors, run your command with an empty `RUSTFLAGS`. For example:

```
RUSTFLAGS= cargo clippy
```

This is because `shell.nix` sets `RUSTFLAGS` to `-D warnings`,
which tells clippy to treat warnings as errors.

## old php stuff...

Only Graham needs to do this, since I run the only remaining PHP
components.

```php
<?php

require_once __DIR__ . '/vendor/autoload.php';
use PhpAmqpLib\Connection\AMQPSSLConnection;
use PhpAmqpLib\Message\AMQPMessage;

function rabbitmq_conn($timeout = 3) {
    $host = 'events.nix.gsc.io';
    $connection = new AMQPSSLConnection(
        $host, 5671,
        'eventsuser, eventspassword, '/',
        array(
            'verify_peer' => true,
            'verify_peer_name' => true,
            'peer_name' => $host,
            'verify_depth' => 10,
            'ca_file' => '/etc/ssl/certs/ca-certificates.crt',
        ), array(
            'connection_timeout' => $timeout,
        )
    );

    return $connection;
}

function gh_secret() {
    return "github webhook secret";
}


```
