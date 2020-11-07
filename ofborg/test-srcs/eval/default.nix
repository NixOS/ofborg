let
  builder = builtins.storePath <ofborg-test-bash>;
in

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
}
