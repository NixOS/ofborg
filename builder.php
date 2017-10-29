<?php

require __DIR__ . '/config.php';
use PhpAmqpLib\Message\AMQPMessage;

# define('AMQP_DEBUG', true);
$connection = rabbitmq_conn();
$channel = $connection->channel();
$channel->basic_qos(null, 1, true);

list($queueName, , ) = $channel->queue_declare('build-results',
                                               false, true, false, false);

list($queueName, , ) = $channel->queue_declare('build-inputs-' . NIX_SYSTEM,
                                               false, true, false, false);
$channel->queue_bind($queueName, 'build-jobs');


function runner($msg) {
    echo "got a job!\n";
    $body = json_decode($msg->body);
    $in = $body->payload;

    $co = new GHE\Checkout(WORKING_DIR, "builder");
    $pname = $co->checkOutRef($in->repository->full_name,
                              $in->repository->clone_url,
                              $in->issue->number,
                              "origin/master"
    );

    $patch_url = $in->issue->pull_request->patch_url;
    echo "Building $patch_url\n";
    $co->applyPatches($pname, $patch_url);

    if ($body->build_default) {
        echo "building via nix-build .\n";

        $cmd = 'NIX_PATH=nixpkgs=%s nix-build --argstr system %s --option restrict-eval true --keep-going .';
        $args = [$pname, NIX_SYSTEM];
    } else {
        echo "building via nix-build . -A\n";
        $attrs = array_intersperse(array_values((array)$body->attrs), '-A');
        var_dump($attrs);

        $fillers = implode(" ", array_fill(0, count($attrs), '%s'));

        $cmd = 'NIX_PATH=nixpkgs=%s nix-build --argstr system %s --option restrict-eval true --keep-going . ' . $fillers;
        $args = $attrs;
        array_unshift($args, NIX_SYSTEM);
        array_unshift($args, $pname);
    }

    try {
        $output = GHE\Exec::exec($cmd, $args);
        $pass = true;
    } catch (GHE\ExecException $e) {
        $output = $e->getOutput();
        $pass = false;
    }

    $lastlines = array_reverse(
        array_slice(
            array_reverse($output),
            0, 10
        )
    );

    $forward = [
        'system' => NIX_SYSTEM,
        'payload' => $in,
        'output' => $lastlines,
        'success' => $pass,
    ];

    $message = new AMQPMessage(json_encode($forward),
                               array('content_type' => 'application/json'));
    $msg->delivery_info['channel']->basic_publish($message, '', 'build-results');
    $msg->delivery_info['channel']->basic_ack($msg->delivery_info['delivery_tag']);

    echo "finished\n";
}

function array_intersperse($array, $val) {
    return array_reduce($array,
                        function($c, $elem) use ($val) {
                            $c[] = $val;
                            $c[] = $elem;
                            return $c;
                        },
                        array());
}


function outrunner($msg) {
    try {
        return runner($msg);
    } catch (\GHE\ExecException $e) {
        var_dump($e->getMessage());
        var_dump($e->getCode());
        var_dump($e->getOutput());
        $msg->delivery_info['channel']->basic_ack($msg->delivery_info['delivery_tag']);
    }
}


$consumerTag = 'consumer' . getmypid();
$channel->basic_consume($queueName, $consumerTag, false, false, false, false, 'outrunner');
while(count($channel->callbacks)) {
    $channel->wait();
}
