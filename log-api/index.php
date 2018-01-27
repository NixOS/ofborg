<?php

header('Content-Type: application/json');
$d = array('attempts' => []);

$root = "/var/lib/nginx/ofborg/";

function abrt($msg) {
    echo $msg;
    exit;
}

if (!is_dir($root)) {
    abrt("root missing");
}

if (!isset($_SERVER['REQUEST_URI']) || empty($_SERVER['REQUEST_URI'])) {
    abrt("uri missing");
}

$reqd = $_SERVER['REQUEST_URI'];
$req = realpath("$root/$reqd");
$serve_root = "https://logs.nix.gsc.io/$reqd";

if ($req === false) {
    abrt("absent");
}

if (strpos($req, $root) !== 0) {
    abrt("bad path");
}

if (!is_dir($req)) {
    abrt("non dir");
}

if ($handle = opendir($req)) {
    while (false !== ($entry = readdir($handle))) {
        if ($entry != "." && $entry != "..") {
            if (is_dir($req . '/' . $entry)) {
                abrt("dir found");
            }

            if (is_file($req . '/' . $entry)) {
                $d['attempts'][$entry] = [ "log_url" => "$serve_root/$entry" ];
            }
        }
    }
    closedir($handle);
}


echo json_encode($d);