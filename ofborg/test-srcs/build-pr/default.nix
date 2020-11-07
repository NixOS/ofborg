let
  builder = builtins.storePath <ofborg-test-bash>;
in {
  success = derivation {
    name = "success";
    system = builtins.currentSystem;
    inherit builder;
    args = ["-c" "echo hi; printf '1\n2\n3\n4\n'; echo ${toString builtins.currentTime} > $out" ];
  };

  failed = derivation {
    name = "failed";
    system = builtins.currentSystem;
    inherit builder;
    args = ["-c" "echo hi; echo ${toString builtins.currentTime}" ];
  };

  sandbox-violation = derivation {
    name = "sandbox-violation";
    system = builtins.currentSystem;
    inherit builder;
    args = ["-c" "echo hi; echo ${toString builtins.currentTime} > $out" ];
    src = ./../../src;
  };
}
