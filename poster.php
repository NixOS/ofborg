<?php

require __DIR__ . '/config.php';
use PhpAmqpLib\Message\AMQPMessage;

# define('AMQP_DEBUG', true);
$connection = rabbitmq_conn();
$channel = $connection->channel();
$channel->basic_qos(null, 1, true);


$channel->exchange_declare('build-jobs', 'fanout', false, true, false);

list($queueName, , ) = $channel->queue_declare('build-results',
                                               false, true, false, false);
$channel->queue_bind($queueName, 'build-jobs');

function runner($msg) {
    $body = json_decode($msg->body);
    $in = $body->payload;

    $num = $in->issue->number;
    if ($body->success) {
        echo "yay! $num passed!\n";
    } else {
        echo "Yikes, $num failed\n";
    }

    reply_to_issue($in, implode("\n", $body->output), $body->success, $body->system);

    var_dump($body->success);

    $msg->delivery_info['channel']->basic_ack($msg->delivery_info['delivery_tag']);
}

function reply_to_issue($issue, $output, $success, $system) {
    $num = $issue->issue->number;
    $owner = $issue->repository->owner->login;
    $repo = $issue->repository->name;
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

    $sha = $pr['head']['sha'];
    echo "Latest sha: $sha\n";
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
    try {
        return runner($msg);
    } catch (GHE\ExecException $e) {
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
