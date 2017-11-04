# grahamcofborg

1. All github events go in to web/index.php, which sends the event to
   an exchange named for the full name of the repo (ex: nixos/nixpkgs)
   in lower case. The exchange is set to "fanout"
2. build-filter.php creates a queue called build-inputs and binds it
   to the nixos/nixpkgs exchange. It also creates an exchange,
   build-jobs, set to fan out. It listens for messages on the
   build-inputs queue. Issue comments from authorized users on
   PRs get tokenized and turned in to build instructions. These jobs
   are then written to the build-jobs exchange.
3. builder.php creates a queue called `build-inputs-x86_64-linux`, and
   binds it to the build-jobs exchange. It then listens for build
   instructions on the `build-inputs-x86_64-linux` queue. For each
   job, it uses nix-build to run the build instructions. The status
   result (pass/fail) and the last ten lines of output are then placed
   in to the `build-results` queue.
4. poster.php declares the build-results queue, and listens for
   messages on it. It posts the build status and text output on the PR
   the build is from.


## Getting Started

 - you'll need to create the `WORKING_DIR`
 - nix-shell
 - composer install
 - php builder.php

The conspicuously missing config.php looks like:



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
