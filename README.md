# grahamcofborg

## Guidelines

1. make sure you've reviewed the code before you trigger it on a PR
   that isn't your own
2. be gentle, preferably don't run mass rebuilds / massive builds like
   chromium on it

## Commands

1. To trigger the bot, the comment _must_ start with a case
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

This will _not_ work:

```
looks good to me!
@grahamcofborg build list of attrs
```

Also this is bad:

```
@grahamcofborg build list of attrs
looks good to me!
```

as it'll try to build `list` `of` `attrs` `looks` `good` `to` `me!`.


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



## old php stuff...

```php
<?php

require_once __DIR__ . '/vendor/autoload.php';
use PhpAmqpLib\Connection\AMQPSSLConnection;
use PhpAmqpLib\Message\AMQPMessage;

define("NIX_SYSTEM", "x86_64-linux");
define("WORKING_DIR", "/home/grahamc/.nix-test");

function rabbitmq_conn() {
    $connection = new AMQPSSLConnection(
        'events.nix.gsc.io', 5671,
        eventsuser, eventspasswordd, '/', array(
            'verify_peer' => true,
            'verify_peer_name' => true,
            'peer_name' => 'events.nix.gsc.io',
            'verify_depth' => 10,
            'ca_file' => '/etc/ssl/certs/ca-certificates.crt'
        )
    );

    return $connection;
}

/*
# Only leader machines (ie: graham's) need this:
function gh_client() {
    $client = new \Github\Client();
    $client->authenticate('githubusername',
                          'githubpassword',
                          Github\Client::AUTH_HTTP_PASSWORD);

    return $client;
}
*/
```
