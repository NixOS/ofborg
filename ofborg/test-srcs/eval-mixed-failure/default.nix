let
  fetchGit = builtins.fetchGit or (path: assert builtins.trace ''
    error: access to path '/fake' is forbidden in restricted mode
  '' false; path);

  builder = builtins.storePath <ofborg-test-bash>;
in

{ nixpkgs ? fetchGit /fake }:

rec {
  success = derivation {
    name = "success";
    system = builtins.currentSystem;
    inherit builder;
    args = ["-c" "echo hi; echo ${toString builtins.currentTime} > $out" ];
  };

  failed = derivation {
    name = "failed";
    system = builtins.currentSystem;
    inherit builder;
    args = ["-c" "echo hi; echo ${toString builtins.currentTime}; echo ${success}" ];
  };

  passes-instantiation = derivation {
    name = "passes-instantiation";
    system = builtins.currentSystem;
    inherit builder;
    args = ["-c" "echo this ones cool" ];
  };

  nixpkgs-restricted-mode = derivation {
    name = "nixpkgs-restricted-mode-fetchgit";
    system = builtins.currentSystem;
    inherit builder;
    args = ["-c" "echo hi; echo ${toString nixpkgs} > $out" ];
  };

  fails-instantiation = assert builtins.trace ''
    You just can't frooble the frozz on this particular system.
  '' false; {};
}
