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

  passes-instantiation = derivation {
    name = "passes-instantiation";
    system = builtins.currentSystem;
    builder = nix.shell;
    args = [
      "-c"
      "echo this ones cool" ];
  };

  fails-instantiation = assert builtins.trace ''
    You just can't frooble the frozz on this particular system.
  '' false; {};
}
