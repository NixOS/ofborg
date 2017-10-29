<?php

require __DIR__ . '/config.php';
use PhpAmqpLib\Message\AMQPMessage;

# define('AMQP_DEBUG', true);
$connection = rabbitmq_conn();
$channel = $connection->channel();
$channel->basic_qos(null, 1, true);

list($queueName, , ) = $channel->queue_declare('build-results',
                                               false, true, false, false);

function runner($msg) {
    $body = json_decode($msg->body);
    $in = $body->payload;

    $num = $in->issue->number;
    if ($body->success) {
        echo "yay! $num passed!\n";
    } else {
        echo "Yikes, $num failede\n";
    }

    reply_to_issue($in, implode("\n", $body->output), $body->success, $body->system);

    var_dump($body->success);

    $msg->delivery_info['channel']->basic_ack($msg->delivery_info['delivery_tag']);
}

function reply_to_issue($issue, $output, $success, $system) {
    $client = gh_client();
    $pr = $client->api('pull_request')->show(
        $issue->repository->owner->login,
        $issue->repository->name,
        $issue->issue->number
    );
    $sha = $pr['head']['sha'];

    $client->api('pull_request')->reviews()->create(
        $issue->repository->owner->login,
        $issue->repository->name,
        $issue->issue->number,
        array(
            'body' => "For system: $system\n\n```\n$output\n```",
            'event' => $success ? 'APPROVE' : 'COMMENT',
            'commit_id' => $sha,
        ));
}


function outrunner($msg) {
    try {
        return runner($msg);
    } catch (ExecException $e) {
        var_dump($e->getMessage());
        var_dump($e->getCode());
        var_dump($e->getOutput());
    }
}


$consumerTag = 'consumer' . getmypid();
$channel->basic_consume($queueName, $consumerTag, false, false, false, false, 'outrunner');
while(count($channel->callbacks)) {
    $channel->wait();
}
