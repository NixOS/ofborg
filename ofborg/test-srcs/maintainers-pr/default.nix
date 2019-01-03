{ ... }:
{
  lib = import ./lib;
  foo.bar.packageA = {
    name = "Hi";
    meta.maintainers = [{ github = "test"; }];
  };
}
