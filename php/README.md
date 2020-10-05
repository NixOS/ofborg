# Webhook Receiver

This PHP code receives the GitHub webhook, checks them for integrity and publishes messages on rabbitmq.


## Configuration

The code expects a `config.php` in it's parent directory. An example configuration looks like this:

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
