<?php

require __DIR__ . '/config.php';
use PhpAmqpLib\Message\AMQPMessage;

# define('AMQP_DEBUG', true);
$connection = rabbitmq_conn();
$channel = $connection->channel();
$channel->basic_qos(null, 1, true);


$channel->exchange_declare('build-results', 'fanout', false, true, false);
$channel->queue_bind('build-results', 'build-results', '');

list($queueName, , ) = $channel->queue_declare('build-results',
                                               false, true, false, false);

function runner($msg) {
    $body = json_decode($msg->body);

    $num = $body->pr->number;
    if ($body->success) {
        echo "yay! $num passed!\n";
    } else {
        echo "Yikes, $num failed\n";
    }

    reply_to_issue($body, implode("\n", $body->output), $body->success, $body->system);

    var_dump($body->success);

    $msg->delivery_info['channel']->basic_ack($msg->delivery_info['delivery_tag']);
}

function reply_to_issue($body, $output, $success, $system) {
    $num = $body->pr->number;
    $owner = $body->repo->owner;
    $repo = $body->repo->name;
    $event = $success ? 'APPROVE' : 'COMMENT';
    $passfail = $success ? "Success" : "Failure";

    echo "Sending $event to $owner/$repo#$num with " . $passfail . " on $system\n";

    $client = gh_client();
    $pr = $client->api('pull_request')->show(
        $owner,
        $repo,
        $num
    );

    if ($pr['state'] == 'closed') {
        $event = 'COMMENT';
    }

    // With multiple archs, it is better to not approve at all, since
    // another arch may come in later with a failure.
    // - By request of Domen
    $event = 'COMMENT';

    $sha = $body->pr->head_sha;
    echo "On sha: $sha\n";
    echo "Body:\n";
    echo $output;
    echo "\n\n";

    $client->api('pull_request')->reviews()->create(
        $owner,
        $repo,
        $num,
        array(
            'body' => "$passfail for system: $system\n\n```\n$output\n```",
            'event' => $event,
            'commit_id' => $sha,
        ));
}


function outrunner($msg) {
    return runner($msg);
}


$consumerTag = 'consumer' . getmypid();
$channel->basic_consume($queueName, $consumerTag, false, false, false, false, 'outrunner');
while(count($channel->callbacks)) {
    $channel->wait();
}
