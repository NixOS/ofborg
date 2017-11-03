<?php

require __DIR__ . '/config.php';

use PhpAmqpLib\Message\AMQPMessage;

# define('AMQP_DEBUG', true);
$connection = rabbitmq_conn();
$channel = $connection->channel();
$channel->basic_qos(null, 1, true);


$channel->queue_declare('mass-rebuild-check-jobs',
                                               false, true, false, false);
list($queueName, , ) = $channel->queue_declare('mass-rebuild-check-inputs',
                                               false, true, false, false);
$channel->queue_bind($queueName, 'github-events', 'pull_request.nixos/nixpkgs');

echo "hi\n";

function outrunner($msg) {
    try {
        runner($msg);
    } catch (\GHE\ExecException $e) {
        var_dump($msg);
        var_dump($e->getMessage());
        var_dump($e->getCode());
        var_dump($e->getOutput());
    } catch (\PhpAmqpLib\Exception\AMQPProtocolChannelException $e) {
        echo "Channel exception:\n";
        var_dump($e);
    }
}

function runner($msg) {
    $in = json_decode($msg->body);

    try {
        $etype = \GHE\EventClassifier::classifyEvent($in);

        if ($etype != "pull_request") {
            echo "Skipping event type: $etype\n";
            $msg->delivery_info['channel']->basic_ack($msg->delivery_info['delivery_tag']);
            return true;
        }
    } catch (\GHE\EventClassifierUnknownException $e) {
        echo "Skipping unknown event type\n";
        print_r($in);
        $msg->delivery_info['channel']->basic_ack($msg->delivery_info['delivery_tag']);
        return true;
    }

    if (!\GHE\ACL::isRepoEligible($in->repository->full_name)) {
        echo "Repo not authorized (" . $in->repository->full_name . ")\n";
        $msg->delivery_info['channel']->basic_ack($msg->delivery_info['delivery_tag']);
        return true;
    }

    if ($in->pull_request->state != "open") {
        echo "PR isn't open in the event\n";
        $msg->delivery_info['channel']->basic_ack($msg->delivery_info['delivery_tag']);
        return true;
    }

    $client = gh_client();
    $status = $client->api('pull_request')->show(
        $in->repository->owner->login,
        $in->repository->name,
        $in->number);
    if ($status['mergeable'] === false) {
        echo "github says the PR isn't able to be merged\n";
        $msg->delivery_info['channel']->basic_ack($msg->delivery_info['delivery_tag']);
        return true;
    }
    if ($status['state'] !== 'open') {
        echo "github says the PR isn't open\n";
        $msg->delivery_info['channel']->basic_ack($msg->delivery_info['delivery_tag']);
        return true;
    }


    $ok_events = [
        'opened',
        'created',
        'edited',
        'synchronize',
        'reopened',
    ];

    if (!in_array($in->action, $ok_events)) {
        echo "Uninteresting event " . $in->action . "\n";
        $msg->delivery_info['channel']->basic_ack($msg->delivery_info['delivery_tag']);
        return true;
    }

    $forward = [
        'original_payload' => $in,
        'repo' => [
            'owner' => $in->repository->owner->login,
            'name' => $in->repository->name,
            'full_name' => $in->repository->full_name,
            'clone_url' => $in->repository->clone_url,
        ],
        'pr' => [
            'number' => $in->number,
            'target_branch' => $in->pull_request->base->ref,
            'patch_url' => $in->pull_request->patch_url,
            'head_sha' => $in->pull_request->head->sha,
        ],
    ];


    echo "forwarding to mass-rebuild-check-jobs :)\n";

    $message = new AMQPMessage(json_encode($forward),
                               array(
                                   'content_type' => 'application/json',
                                   'delivery_mode' => AMQPMessage::DELIVERY_MODE_PERSISTENT,
                               ));
    $msg->delivery_info['channel']->basic_publish($message, '', 'mass-rebuild-check-jobs');
    $msg->delivery_info['channel']->basic_ack($msg->delivery_info['delivery_tag']);
    return true;
}

$consumerTag = 'massrebuildcheckfilter' . getmypid();
$channel->basic_consume($queueName, $consumerTag, false, false, false, false, 'outrunner');
while(count($channel->callbacks)) {
    $channel->wait();
}

echo "Bye\n";