<?php

require __DIR__ . '/config.php';

use PhpAmqpLib\Message\AMQPMessage;

# define('AMQP_DEBUG', true);
$connection = rabbitmq_conn();
$channel = $connection->channel();
$channel->basic_qos(null, 1, true);


list($queueName, , ) = $channel->queue_declare('mass-rebuild-check-jobs',
                                               false, true, false, false);

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

    $client = gh_client();
    $status = $client->api('pull_request')->show(
        $in->repo->owner,
        $in->repo->name,
        $in->pr->number);
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

    $against_name = "origin/" . $in->pr->target_branch;
    echo "Building against $against_name\n";
    $co = new GHE\Checkout(WORKING_DIR, "mr-est");
    $pname = $co->checkOutRef($in->repo->full_name,
                              $in->repo->clone_url,
                              $in->pr->number,
                              $against_name
    );

    $against = GHE\Exec::exec('git rev-parse %s', [$against_name]);
    echo " $against_name is $against[0]\n";

    try {
        $co->applyPatches($pname, $in->pr->patch_url);
    } catch (GHE\ExecException $e) {
        echo "Received ExecException applying patches, likely due to conflicts:\n";
        var_dump($e->getCode());
        var_dump($e->getMessage());
        var_dump($e->getArgs());
        var_dump($e->getOutput());
        $msg->delivery_info['channel']->basic_ack($msg->delivery_info['delivery_tag']);
        return false;
    }

    $current = GHE\Exec::exec('git rev-parse HEAD');
    echo " currently at ${current[0]}\n";


    reply_to_issue($in->repo, $in->pr, $against[0], $current[0]);
    $msg->delivery_info['channel']->basic_ack($msg->delivery_info['delivery_tag']);
    return true;
}

function reply_to_issue($repo, $pr, $prev, $current) {
    $client = gh_client();

    echo "current labels:\n";
    $already_there = $client->api('issue')->labels()->all(
        $repo->owner,
        $repo->name,
        $pr->number);
    $already_there = array_map(function($val) { return $val['name']; }, $already_there);
    var_dump($already_there);

    $output = GHE\Exec::exec('$(nix-instantiate --eval -E %s) %s %s',
                             [
                                 '<nixpkgs/maintainers/scripts/rebuild-amount.sh>',
                                 $prev,
                                 $current
                             ]
    );

    $labels = GHE\RebuildTagClassifier::parseAndLabel($output);

    foreach ($labels as $label) {
        if (in_array($label, $already_there)) {
            echo "already labeled $label\n";

            continue;
        } else {
            echo "will label +$label\n";
        }

        $client->api('issue')->labels()->add(
            $repo->owner,
            $repo->name,
            $pr->number,
            $label);
    }
}

$consumerTag = 'consumer' . getmypid();
$channel->basic_consume($queueName, $consumerTag, false, false, false, false, 'outrunner');
while(count($channel->callbacks)) {
    $channel->wait();
}

echo "Bye\n";