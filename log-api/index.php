<?php

header('Content-Type: application/json');
$d = array('attempts' => []);

$root = "/var/log/ofborg/";

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

$reqd = substr($_SERVER['REQUEST_URI'], strlen("/logs/"));
$req = realpath("$root/$reqd");
$serve_root = "https://logs.nix.ci/logfile/$reqd";

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
                if (substr($entry, -strlen(".metadata.json"),strlen(".metadata.json")) == ".metadata.json") {
                    $metadata = json_decode(file_get_contents($req . '/' . $entry), JSON_OBJECT_AS_ARRAY);
                    $attempt = $metadata['attempt_id'];
                    if (!isset($d['attempts'][$attempt])) {
                        $d['attempts'][$attempt] = [];
                    }
                    $d['attempts'][$attempt]['metadata'] = $metadata;
                } elseif (substr($entry, -strlen(".result.json"),strlen(".result.json")) == ".result.json") {
                    $metadata = json_decode(file_get_contents($req . '/' . $entry), JSON_OBJECT_AS_ARRAY);
                    $attempt = $metadata['attempt_id'];
                    if (!isset($d['attempts'][$attempt])) {
                        $d['attempts'][$attempt] = [];
                    }
                    $d['attempts'][$attempt]['result'] = $metadata;

                } else {
                    if (!isset($d['attempts'][$entry])) {
                        $d['attempts'][$entry] = [];
                    }
                    $d['attempts'][$entry]['log_url'] = "$serve_root/$entry";
                }
            }
        }
    }
    closedir($handle);
}


echo json_encode($d);