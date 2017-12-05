<?php

namespace GHE;

class RebuildTagClassifier {

    public static function parseAndLabel($text) {
        $counts = self::parse($text);

        if (!isset($counts['x86_64-darwin'])) {
            $counts['x86_64-darwin'] = 0;
        }

        if (!isset($counts['x86_64-linux'])) {
            $counts['x86_64-linux'] = 0;
        }

        $labels = [];
        foreach ($counts as $arch => $count) {
            $label[] = self::labelForArchCount($arch, $count);
        }

        return $label;
    }

    public static function parse($output) {
        $counts = [];
        foreach ($output as $line) {
            if (preg_match('/^\s*(\d+) (.*)$/', $line, $matches)) {
                $counts[$matches[2]] = (int)$matches[1];
            }
        }

        return $counts;
    }

    public static function labelForArchCount($arch, $count) {
        if ($arch == "x86_64-linux") {
            $prefix = "10.rebuild-linux: ";
        } elseif ($arch == "x86_64-darwin") {
            $prefix = "10.rebuild-darwin: ";
        } else {
            throw new RebuildTagClassifierArchException("Unknown arch $arch");
        }

        if ($count > 500) {
            $suffix = "501+";
        } else if ($count > 100) {
            $suffix = "101-500";
        } else if ($count > 10) {
            $suffix = "11-100";
        } else if ($count > 0) {
            $suffix = "1-10";
        } else {
            $suffix = "0";
        }

        return $prefix . $suffix;
    }
}

class RebuildTagClassifierArchException extends \Exception {}