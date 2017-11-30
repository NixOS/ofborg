{ pkgs ? import ./nix {}
}:
{
  ofborg.rs = (pkgs.callPackage ./nix/ofborg-carnix.nix {}).ofborg_0_1_0;
}
