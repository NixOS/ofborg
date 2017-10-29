<?php

require __DIR__ . '/config.php';

use PhpAmqpLib\Message\AMQPMessage;

# define('AMQP_DEBUG', true);
$connection = rabbitmq_conn();
$channel = $connection->channel();
$channel->basic_qos(null, 1, true);


list($queueName, , ) = $channel->queue_declare('mass-rebuild-checks',
                                               false, true, false, false);
$channel->queue_bind($queueName, 'nixos/nixpkgs');

echo "hi\n";

function outrunner($msg) {
    try {
        $ret = runner($msg);
        var_dump($ret);
        if ($ret === true) {
            echo "cool\n";
            echo "acking\n";
            $r = $msg->delivery_info['channel']->basic_ack($msg->delivery_info['delivery_tag']);
            var_dump($r);
            echo "acked\n";
        } else {
            echo "Not acking?\n";
        }
    } catch (\GHE\ExecException $e) {
        var_dump($msg);
        var_dump($e->getMessage());
        var_dump($e->getCode());
        var_dump($e->getOutput());
    }
}

function runner($msg) {
    $in = json_decode($msg->body);

    try {
        $etype = \GHE\EventClassifier::classifyEvent($in);

        if ($etype != "pull_request") {
            echo "Skipping event type: $etype\n";
            return true;
        }
    } catch (\GHE\EventClassifierUnknownException $e) {
        echo "Skipping unknown event type\n";
        print_r($in);
        return true;
    }

    if (!\GHE\ACL::isRepoEligible($in->repository->full_name)) {
        echo "Repo not authorized (" . $in->repository->full_name . ")\n";
        return true;
    }

    if ($in->pull_request->state != "open") {
        echo "PR isn't open\n";
        return true;
    }

    $ok_events = [
        'created',
        'edited',
        'synchronize',
    ];

    if (!in_array($in->action, $ok_events)) {
        echo "Uninteresting event " . $in->action . "\n";
        return true;
    }

    $r = $msg->delivery_info['channel']->basic_ack($msg->delivery_info['delivery_tag']);
    var_dump($r);
    echo "acked\n";


        $against_name = "origin/" . $in->pull_request->base->ref;
        echo "Building against $against_name\n";
        $co = new GHE\Checkout("/home/grahamc/.nix-test", "mr-est");
    $pname = $co->checkOutRef($in->repository->full_name,
            $in->repository->clone_url,
            $in->number,
            $against_name
            );

    $against = GHE\Exec::exec('git rev-parse %s', [$against_name]);
    echo " $against_name is $against[0]\n";

    try {
        $co->applyPatches($pname, $in->pull_request->patch_url);
    } catch (GHE\ExecException $e) {
        echo "Received ExecException applying patches, likely due to conflicts:\n";
        var_dump($e->getCode());
        var_dump($e->getMessage());
        var_dump($e->getArgs());
        var_dump($e->getOutput());
        return false;
    }

    $current = GHE\Exec::exec('git rev-parse HEAD');
    echo " currently at ${current[0]}\n";


    reply_to_issue($in, $against[0], $current[0]);
    $msg->delivery_info['channel']->basic_cancel($msg->delivery_info['consumer_tag']);
    return false;
}

function reply_to_issue($issue, $prev, $current) {
    $client = gh_client();

    echo "current labels:\n";
    $already_there = $client->api('issue')->labels()->all(
        $issue->repository->owner->login,
        $issue->repository->name,
        $issue->number);
    $already_there = array_map(function($val) { return $val['name']; }, $already_there);
    var_dump($already_there);

    $output = GHE\Exec::exec('$(nix-instantiate --eval -E %s) %s %s',
                             [
                                 '<nixpkgs/maintainers/scripts/rebuild-amount.sh>',
                                 $prev,
                                 $current
                             ]
    );

    $labels = [];
    foreach ($output as $line) {
        if (preg_match('/^\s*(\d+) (.*)$/', $line, $matches)) {
            var_dump($matches);
            # TODO: separate out the rebuild ranges from the rebuild platform and
            # splice the string together, rather than this ugliness
            if ($matches[1] > 500) {
                if ($matches[2] == "x86_64-darwin") {
                    $labels[] = "10.rebuild-darwin: 501+";
                } else {
                    $labels[] = "10.rebuild-linux: 501+";
                }
            } else if ($matches[1] > 100 && $matches[1] <= 500) {
                if ($matches[2] == "x86_64-darwin") {
                    $labels[] = "10.rebuild-darwin: 101-500";
                } else {
                    $labels[] = "10.rebuild-linux: 101-500";
                }
            } else if ($matches[1] > 10 && $matches[1] <= 100) {
                if ($matches[2] == "x86_64-darwin") {
                    $labels[] = "10.rebuild-darwin: 11-100";
                } else {
                    $labels[] = "10.rebuild-linux: 11-100";
                }
            } else if ($matches[1] <= 10) {
                if ($matches[2] == "x86_64-darwin") {
                    $labels[] = "10.rebuild-darwin: 1-10";
                } else {
                    $labels[] = "10.rebuild-linux: 1-10";
                }
            }
        }
    }


    foreach ($labels as $label) {
        if (in_array($label, $already_there)) {
            echo "already labeled $label\n";

            continue;
        } else {
            echo "will label +$label\n";
        }

        $client->api('issue')->labels()->add(
            $issue->repository->owner->login,
            $issue->repository->name,
            $issue->number,
            $label);
    }
}

$consumerTag = 'consumer' . getmypid();
$channel->basic_consume($queueName, $consumerTag, false, true, false, false, 'outrunner');
while(count($channel->callbacks)) {
    $channel->wait();
}

echo "Bye\n";