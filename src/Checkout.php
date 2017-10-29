<?php

namespace GHE;

class Checkout {

    protected $root;
    protected $type;

    function __construct($root, $type) {
        $this->root = $root;
        $this->type = $type;
    }

    function checkOutRef($repo_name, $clone_url, $id, $ref) {
        $this->prefetchRepoCache($repo_name, $clone_url);

        $pname = $this->pathToRepoCache($repo_name);
        $bname = $this->pathToBuildDir($repo_name, $id);

        $guard = $this->guard($bname);
        if (!is_dir($bname)) {
            echo "Cloning " . $id . " to $bname\n";
            Exec::exec('git clone --reference-if-able %s %s %s',
                       [
                           $pname,
                           $clone_url,
                           $bname
                       ]);
        }

        if (!chdir($bname)) {
            throw new CoFailedException("Failed to chdir to $bname\n");
        }

        echo "fetching " . $id . " in $bname\n";
        Exec::exec('git fetch origin');
        try {
            Exec::exec('git am --abort');
        } catch (ExecException $e) {
            // non-zero exit if no am is in progress
        }
        Exec::exec('git reset --hard %s', [$ref]);


        $this->release($guard);

        return $bname;
    }

    function applyPatches($bname, $patch_url) {
        if (!chdir($bname)) {
            throw new CoFailedException("Failed to chdir to $bname\n");
        }

        $guard = $this->guard($pname);
        Exec::exec('curl -L %s | git am --no-gpg-sign -', [$patch_url]);
        $this->release($guard);
    }

    function prefetchRepoCache($name, $clone_url) {
        if (!chdir($this->root)) {
            throw new CoFailedException("Failed to chdir to " . $this->root);
        }

        $pname = $this->pathToRepoCache($name);

        $guard = $this->guard($pname);

        if (!is_dir($pname)) {
            echo "Cloning " . $name . " to $pname\n";
            Exec::exec('git clone --bare %s %s',
                       [
                           $clone_url,
                           $pname
                       ]);
        }

        $this->release($guard);

        if (!chdir($pname)) {
            throw new CoFailedException("Failed to chdir to $pname");
        }

        echo "Fetching $name to $pname\n";
        Exec::exec('git fetch origin');
    }

    function pathToRepoCache($name) {
        return $this->root . "/repo-" . md5($name);
    }

    function pathToBuildDir($repo, $id_number) {
        $id = (int) $id_number;
        $repo_hash = md5($repo);
        $type = $this->type;

        return $this->root . "/$type-$repo_hash-$id";
    }

    function guard($path) {
        $res = fopen("$path.lock", 'c');
        while (!flock($res, LOCK_EX)) {
            echo "waiting for lock on $path...\n";
            sleep(1);
        }

        return  $res;
    }

    function release($res) {
        fclose($res);
    }

}

class CoFailedException extends \Exception {}