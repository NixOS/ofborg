<?php
namespace GHE;

class ACL {
    static public function getRepos() {
        return [
            'grahamc/elm-stuff',
            'nixos/nixpkgs',
            'nixos/nixpkgs-channels',
        ];
    }

    static public function isRepoEligible($repo) {
        return in_array(strtolower($repo), self::getRepos());
    }
}