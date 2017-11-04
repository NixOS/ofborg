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
        // var_dump($msg);
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


    $head_sha = $in->pr->head_sha;

    $ghclient = gh_client();

    $overallstatus = new GHE\CommitStatus($ghclient, $in->repo->owner,
                                          $in->repo->name, $head_sha,
                                          'grahamcofborg-eval');
    echo "marking PR as pending\n";
    $overallstatus->pending('GrahamCOfBorg is starting');

    $against_name = "origin/" . $in->pr->target_branch;
    echo "Building against $against_name\n";
    $co = new GHE\Checkout(WORKING_DIR, "mr-est");
    $pname = $co->checkOutRef($in->repo->full_name,
                              $in->repo->clone_url,
                              $in->pr->number,
                              $against_name
    );
    $overallstatus->pending('Checked out ' . $against_name);

    $against = GHE\Exec::exec('git rev-parse %s', [$against_name]);
    echo " $against_name is $against[0]\n";

    $prev_darwin_stdenv = identify_stdenv("x86_64-darwin");
    $prev_linux_stdenv = identify_stdenv("x86_64-linux");
    echo "starting stdenvs:\n";
    echo " - darwin: $prev_darwin_stdenv\n";
    echo " - linux: $prev_linux_stdenv\n";

    try {
        $co->applyPatches($pname, $in->pr->patch_url);
        $overallstatus->pending('Applied patches from ' . $in->pr->number);
    } catch (GHE\ExecException $e) {
        echo "marking PR as failed to apply patches\n";
        $overallstatus->error('Failed to apply patches to ' . $against_name);

        echo "Received ExecException applying patches, likely due to conflicts:\n";
        var_dump($e->getCode());
        var_dump($e->getMessage());
        var_dump($e->getArgs());
        var_dump($e->getOutput());
        $msg->delivery_info['channel']->basic_ack($msg->delivery_info['delivery_tag']);
        return false;
    }

    $querypkgsstatus = new GHE\CommitStatus($ghclient, $in->repo->owner,
                                            $in->repo->name, $head_sha,
                                            'grahamcofborg-eval-packagelist');

    $overallstatus->pending('Checking if stdenv changed');
    $new_darwin_stdenv = identify_stdenv("x86_64-darwin");
    $new_linux_stdenv = identify_stdenv("x86_64-linux");
    echo "new stdenvs:\n";
    echo " - darwin: $new_darwin_stdenv\n";
    echo " - linux: $new_linux_stdenv\n";


    $current = GHE\Exec::exec('git rev-parse HEAD');
    echo " currently at ${current[0]}\n";

    $overallstatus->pending('Checking for sub-evals');

    try_eval($ghclient, $in->repo->owner, $in->repo->name, $head_sha,
             'package-list',
             'nix-env --file . --query --available > /dev/null 2>&1', []);

    try_eval($ghclient, $in->repo->owner, $in->repo->name, $head_sha,
             'nixos-options',
             'nix-instantiate ./nixos/release.nix -A options', []);

    try_eval($ghclient, $in->repo->owner, $in->repo->name, $head_sha,
             'nixos-manual',
             'nix-instantiate ./nixos/release.nix -A manual', []);

    try_eval($ghclient, $in->repo->owner, $in->repo->name, $head_sha,
             'nixpkgs-manual',
             'nix-instantiate ./pkgs/top-level/release.nix -A manual', []);

    try_eval($ghclient, $in->repo->owner, $in->repo->name, $head_sha,
             'nixpkgs-tarball',
             'nix-instantiate ./pkgs/top-level/release.nix -A tarball', []);

    try_eval($ghclient, $in->repo->owner, $in->repo->name, $head_sha,
             'nixpkgs-unstable-jobset',
             'nix-instantiate ./pkgs/top-level/release.nix -A unstable', []);

    reply_to_issue($overallstatus, $in->repo, $in->pr,
                   $new_darwin_stdenv !== $prev_darwin_stdenv,
                   $new_linux_stdenv !== $prev_linux_stdenv,
                   $against[0], $current[0]);

    echo "marking PR as success\n";
    $overallstatus->success('Evaluation checks OK');

    $msg->delivery_info['channel']->basic_ack($msg->delivery_info['delivery_tag']);
    return true;
}

function try_eval($ghclient, $owner, $name, $sha, $eval_name, $cmd, $args) {
    echo "Starting $eval_name on $sha\n";

    $status = new GHE\CommitStatus($ghclient, $owner,
                                   $name, $sha,
                                   'grahamcofborg-eval-' . $eval_name);

    $status->pending("starting to run $cmd");

    try {
        GHE\Exec::exec($cmd, $args);
        echo "Success running $eval_name on $sha\n";
        $status->success("Finished running $cmd");
    } catch (GHE\ExecException $e) {
        echo "Failed to run $eval_name on $sha\n";
        $status->pending("Failed to run $cmd");
        return false;
    }
}

function identify_stdenv($arch) {
    $lines = GHE\Exec::exec('nix-instantiate . -A stdenv --argstr system %s 2>&1',
                   [$arch]);
    echo "fetching stdenv for $arch:\n";
    var_dump($lines);
    return array_pop($lines);
}

function reply_to_issue($overallstatus, $repo, $pr, $darwin_changed, $linux_changed, $prev, $current) {
    $client = gh_client();

    echo "current labels:\n";
    $already_there = $client->api('issue')->labels()->all(
        $repo->owner,
        $repo->name,
        $pr->number);
    $already_there = array_map(function($val) { return $val['name']; }, $already_there);
    var_dump($already_there);

    $overallstatus->pending("Diffing derivations");
    $output = GHE\Exec::exec('$(nix-instantiate --eval -E %s) %s %s',
                             [
                                 '<nixpkgs/maintainers/scripts/rebuild-amount.sh>',
                                 $prev,
                                 $current
                             ]
    );

    var_dump($output);

    $labels = GHE\RebuildTagClassifier::parseAndLabel($output);

    if ($darwin_changed) {
        $labels[] = '10.rebuild-darwin-stdenv';
    }
    if ($linux_changed) {
        $labels[] = '10.rebuild-linux-stdenv';
    }

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