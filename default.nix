{ nixpkgs ? import ./nix {} }:
import ./nix/ofborg-carnix.nix {
  inherit (nixpkgs) buildPlatform buildRustCrate fetchgit;
}
