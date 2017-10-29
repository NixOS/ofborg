<?php

namespace GHE;

class Exec {
    public static function exec($cmd, $args = array()) {
        $safeArgs = array_map('escapeshellarg', $args);
        $interiorCmd = vsprintf($cmd, $safeArgs);

        $exteriorCmd = sprintf('/bin/sh -o pipefail -euc %s 2>&1',
                               escapeshellarg($interiorCmd));

        exec($exteriorCmd, $output, $return);

        if ($return > 0) {
            throw new ExecException($cmd, $args, $output, $return);
        }

        return $output;
    }
}

class ExecException extends \Exception {
    protected $args;
    protected $output;

    public function __construct($cmd, $args, $output, $return) {
        $this->args = $args;
        $this->output = $output;

        parent::__construct("Error calling $cmd", $return);
    }

    public function getArgs() {
        return $this->args;
    }

    public function getOutput() {
        return $this->output;
    }

}