<?php

namespace GHE;

class CommitStatus {

    protected $ghclient;
    protected $owner;
    protected $repo;
    protected $sha;
    protected $name;

    function __construct($ghclient, $owner, $repo, $sha, $name) {
        $this->ghclient = $ghclient;
        $this->owner = $owner;
        $this->repo = $repo;
        $this->sha = $sha;
        $this->name = $name;
    }

    public function pending($description) {
        $this->mark('pending', $description);
    }

    public function error($description) {
        $this->mark('error', $description);
    }

    public function failure($description) {
        $this->mark('failure', $description);
    }

    public function success($description) {
        $this->mark('success', $description);
    }

    public function mark($state, $description) {
        $this->ghclient->api('repository')->statuses()->create(
            $this->owner,
            $this->repo,
            $this->sha,
            [
                'state' => $state,
                'context' => $this->name,
                'description' => $description,
            ]
        );
    }
}
