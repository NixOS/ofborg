let
  nix = import <nix/config.nix>;
in rec {
  success = derivation {
    name = "success";
    system = builtins.currentSystem;
    builder = nix.shell;
    args = [
      "-c"
      "echo hi; echo ${toString builtins.currentTime} > $out" ];
  };

  failed = derivation {
    name = "failed";
    system = builtins.currentSystem;
    builder = nix.shell;
    args = [
      "-c"
      "echo hi; echo ${toString builtins.currentTime}; echo ${success}" ];
  };
}
