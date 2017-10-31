<?php

require_once __DIR__ . '/../config.php';
use PhpAmqpLib\Message\AMQPMessage;

$connection = rabbitmq_conn();
$channel = $connection->channel();

$raw = file_get_contents('php://input');
$input = json_decode($raw);
if (!isset($input->repository->full_name)) {
   echo "no full_name set?";
   exit();
} else {
    echo "full_name present\n";
}

$name = strtolower($input->repository->full_name);
if (!GHE\ACL::isRepoEligible($name)) {
    echo "repo not in ok name list";
    exit(1);
} else {
    echo "full_name ok\n";
}

$dec = $channel->exchange_declare('github-events', 'topic', false, true, false);

$message = new AMQPMessage(json_encode($input),
                           array(
                               'content_type' => 'application/json',
                               'delivery_mode' => AMQPMessage::DELIVERY_MODE_PERSISTENT,
                           ));

try {
    $etype = \GHE\EventClassifier::classifyEvent($input);
} catch (\GHE\EventClassifierUnknownException $e) {
    $etype = "unknown";
}

$routing_key = "$etype.$name";
var_dump($routing_key);
$rec = $channel->basic_publish($message, 'github-events', $routing_key);

echo "ok";