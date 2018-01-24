let
  nix = import <nix/config.nix>;
in {
  success = derivation {
    name = "success";
    system = builtins.currentSystem;
    builder = nix.shell;
    args = [
      "-c"
      "for i in `seq 1 10000; do echo $i; sleep 0; done; echo ${toString builtins.currentTime} > $out" ];
  };

  failed = derivation {
    name = "failed";
    system = builtins.currentSystem;
    builder = nix.shell;
    args = [
      "-c"
      "echo hi; echo ${toString builtins.currentTime}" ];
  };

  sandbox-violation = derivation {
    name = "sandbox-violation";
    system = builtins.currentSystem;
    builder = ./../../src;
  };
}
