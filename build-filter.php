<?php

require __DIR__ . '/config.php';
use PhpAmqpLib\Message\AMQPMessage;

# define('AMQP_DEBUG', true);
$connection = rabbitmq_conn();
$channel = $connection->channel();
$channel->basic_qos(null, 1, true);

$channel->exchange_declare('build-jobs', 'fanout', false, true, false);


list($queueName, , ) = $channel->queue_declare('build-inputs',
                                               false, true, false, false);
$channel->queue_bind($queueName, 'github-events', 'issue_comment.nixos/nixpkgs');

function runner($msg) {
    $in = json_decode($msg->body);

    try {
        $etype = \GHE\EventClassifier::classifyEvent($in);

        if ($etype != "issue_comment") {
            echo "Skipping event type: $etype\n";
            return true;
        }
    } catch (\GHE\EventClassifierUnknownException $e) {
        echo "Skipping unknown event type\n";
        print_r($in);
        return true;
    }

    $cmt = explode(' ', strtolower($in->comment->body));
    if (!in_array('@grahamcofborg', $cmt)) {
        echo "not a borgpr\n";
        return true;
    }

    if (!\GHE\ACL::isUserAuthorized($in->comment->user->login)) {
        echo "Commenter not authorized (" . $in->comment->user->login . ")\n";
        return true;
    }

    if (!\GHE\ACL::isRepoEligible($in->repository->full_name)) {
        echo "Repo not authorized (" . $in->repository->full_name . ")\n";
        return true;
    }

    if (!isset($in->issue->pull_request)) {
        echo "not a PR\n";
        return true;
    }

    # // We don't get a useful pull_request here, we'd have to fetch it
    # to know if it is open
    #if ($in->issue->pull_request->state != "open") {
    #    var_dump($in->issue->pull_request);
    #    echo "PR isn't open\n";
    #    return true;
    # }

    $cmt = explode(' ', $in->comment->body);

    $tokens = array_map(function($term) { return trim($term); },
                     array_filter($cmt,
                                  function($term) {
                                      return !in_array(strtolower($term), [
                                          "@grahamcofborg",
                                          "",
                                      ]);
                                  }
                     )
    );

    if (count($tokens) == 1 && implode("", $tokens) == "default") {
        echo "default support is blocked\n";
        return true;
        $forward = [
            'payload' => $in,
            'build_default' => true,
            'attrs' => [],
        ];
    } else {
        $forward = [
            'payload' => $in,
            'build_default' => false,
            'attrs' => $tokens,
        ];
    }

    echo "forwarding to build-jobs :)\n";

    $message = new AMQPMessage(json_encode($forward),
                               array(
                                   'content_type' => 'application/json',
                                   'delivery_mode' => AMQPMessage::DELIVERY_MODE_PERSISTENT,
                               ));
    $msg->delivery_info['channel']->basic_publish($message, 'build-jobs');
    return true;
}


function outrunner($msg) {
    try {
        if (runner($msg) === true) {
            $msg->delivery_info['channel']->basic_ack($msg->delivery_info['delivery_tag']);
        }
    } catch (\GHE\ExecException $e) {
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
