<?php

require_once __DIR__ . '/../config.php';
use PhpAmqpLib\Message\AMQPMessage;

$connection = rabbitmq_conn();
$channel = $connection->channel();

$input = json_decode(file_get_contents('php://input'), true);
if (!isset($input['repository']['full_name'])) {
   echo "no full_name set?";
   exit(0);
}


$name = strtolower($input['repository']['full_name']);
if (!GHE\ACL::isRepoEligible($name)) {
   echo "repo not in ok name list";
   exit(1);
}

$channel->exchange_declare($name, 'fanout', false, true, false);

$message = new AMQPMessage(json_encode($input),
         array('content_type' => 'application/json'));

$channel->basic_publish($message, $name);
