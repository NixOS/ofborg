{ system, ... }:
{
  package = derivation {
    name = "package";
    inherit system;
    builder = "/bin/sh";
    args = [ ];
  };

  unavailable = throw "not available on ${system}";
  scalar = 42;

  linux-only =
    if builtins.match ".*-linux" system != null then
      derivation {
        name = "linux-only";
        inherit system;
        builder = "/bin/sh";
        args = [ ];
      }
    else
      throw "not available on ${system}";

  tests = {
    nested = derivation {
      name = "nested-test";
      inherit system;
      builder = "/bin/sh";
      args = [ ];
    };
  };

  empty = { };
}
