# grahamcofborg

## Guidelines

1. make sure you've reviewed the code before you trigger it on a PR
   that isn't your own
2. be gentle, preferably don't run mass rebuilds / massive builds like
   chromium on it

## Automatic Building

Users who are _trusted_ (see: ./config.public.json) will have their
PRs automatically trigger builds if their commits follow the
well-defined format of Nixpkgs. Example messages and the builds:

|Message|Automatic Build|
|-|-|
|`vim: 1.0.0 -> 2.0.0`|`vim`|
|`python3Packages.requests,python2Packages.requests: 1.0.0 -> 2.0.0`|`python3Packages.requests`, `python2Packages.requests`|
|`python{2,3}Packages.requests: 1.0.0 -> 2.0.0`|_nothing_|



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

This will run `nix-build ./nixos/release.nix -A tests.list -A tests.of -A tests.attrs` in
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


---




# Running a builder

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

function gh_client() {
    $client = new \Github\Client();
    $client->authenticate('githubusername',
                          'githubpassword',
                          Github\Client::AUTH_HTTP_PASSWORD);

    return $client;
}

```
