<?php

ini_set("display_errors", 0);
error_reporting(-1);

ob_start();

require_once __DIR__ . '/../config.php';
use PhpAmqpLib\Message\AMQPMessage;

class DumpableException extends \Exception{}
class InvalidPayloadException extends DumpableException {}
class InvalidSignatureException extends DumpableException {}
class InvalidEventTypeException extends DumpableException {}
class ValidationFailureException extends DumpableException {}
class ExecutionFailureException extends DumpableException {}

function retry_rabbitmq_conn() {
    $maximum_time = 25;
    $delay = 1;
    $timeout = 0.5;

    for ($i = 0.0; $i < $maximum_time; $i += ($timeout + $delay)) {
        try {
            return rabbitmq_conn($timeout);
        } catch (ErrorException $e) {
            trigger_error(print_r($e, true), E_USER_WARNING);
        }
        sleep($delay);
    }
    trigger_error("Failed to connect to RabbitMQ", E_USER_WARNING);
    echo "rabbit failure";
    exit(1);
}


function payload() {
    if (!isset($_SERVER)) {
        throw new InvalidPayloadException('_SERVER undefined');
    }

    if (!isset($_SERVER['CONTENT_TYPE'])) {
        throw new InvalidPayloadException('CONTENT_TYPE not set in _SERVER');
    }

    switch ($_SERVER['CONTENT_TYPE']) {
    case 'application/json':
        $input = file_get_contents('php://input');
        if ($input === false) {
            throw new InvalidPayloadException('Failed to read php://input for application/json');
        } else {
            return $input;
        }
    default:
        throw new InvalidPayloadException('Unsupported content type: ' . $_SERVER['CONTENT_TYPE']);
    }
}

function signature() {
    if (!isset($_SERVER)) {
        throw new InvalidSignatureException('_SERVER undefined');
    }

    if (!isset($_SERVER['HTTP_X_HUB_SIGNATURE'])) {
        throw new InvalidSignatureException('HTTP_X_HUB_SIGNATURE absent from _SERVER');
    }

    return $_SERVER['HTTP_X_HUB_SIGNATURE'];
}

function event_type() {
    if (!isset($_SERVER)) {
        throw new InvalidEventTypeException('_SERVER undefined');
    }

    if (!isset($_SERVER['HTTP_X_GITHUB_EVENT'])) {
        throw new InvalidEventTypeException('HTTP_X_GITHUB_EVENT absent from _SERVER');
    }

    $type = trim($_SERVER['HTTP_X_GITHUB_EVENT']);

    if (strlen($type) === 0) {
        throw new InvalidEventTypeException('After trimming, event type is zero-length');
    }

    return $type;
}

function validate_payload_signature($secret, $payload, $signature) {
    if (!extension_loaded('hash')) {
        throw new ValidationFailureException('Missing hash extension');
    }

    $components = explode('=', $signature, 2);
    if (count($components) != 2) {
        throw new ValidationFailureException('Provided signature seems invalid after splitting on =');
    }

    $algo = $components[0];
    $provided_hash = $components[1];

    if (!in_array($algo, hash_algos(), true)) {
        throw new ValidationFailureException("Hash algorithm '$algo' is not supported by the extension.");
    }

    $ok_algos = [
        'sha1',
        'sha256',
        'sha512',
    ];
    if (!in_array($algo, $ok_algos, true)) {
        throw new ValidationFailureException("Hash algorithm '$algo' is not considered okay");
    }

    $calculated_hash = hash_hmac($algo, $payload, $secret);

    return hash_equals($provided_hash, $calculated_hash);
}

try {
    $raw = payload();
    if (!validate_payload_signature(gh_secret(), $raw, signature())) {
        throw new ExecutionFailureException('Failed to validate signature');
    }

    $input = json_decode($raw);
    if ($input === null) {
        throw new ExecutionFailureException('Failed to decode the JSON');
    }

    if (!isset($input->repository)) {
        throw new ExecutionFailureException('Dataset does not have a repository');
    }

    if (!isset($input->repository->full_name)) {
        throw new ExecutionFailureException('Dataset repository does not have a name');
    }

    $name = strtolower($input->repository->full_name);
    $eventtype = event_type();

    $connection = retry_rabbitmq_conn();
    $channel = $connection->channel();

    $dec = $channel->exchange_declare(
        'github-events',
        'topic',
        false, // passive
        true, // durable
        false // auto_delete
    );

    $channel->queue_declare(
        'github-events-unknown',
        false, // passive
        true, // durable
        false, // exclusive
        false // auto-delete
    );
    $channel->queue_bind(
        'github-events-unknown',
        'github-events',
        'unknown.*'
    );

    $message = new AMQPMessage(json_encode($input),
                               array(
                                   'content_type' => 'application/json',
                                   'delivery_mode' => AMQPMessage::DELIVERY_MODE_PERSISTENT,
                               ));

    $routing_key = "$eventtype.$name";
    $rec = $channel->basic_publish($message, 'github-events', $routing_key);

    echo "ok";
} catch (DumpableException $e) {
    trigger_error(print_r($e, true), E_USER_WARNING);
    header("HTTP/1.1 400 Eh", true, 400);
    var_dump($e);
    echo ob_get_clean();
} catch (\Exception $e) {
    trigger_error(print_r($e, true), E_USER_WARNING);
    header("HTTP/1.1 400 Meh", true, 400);
    var_dump(get_class($e));
    echo ob_get_clean();
}