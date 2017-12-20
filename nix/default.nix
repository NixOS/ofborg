let
  hostpkgs = import <nixpkgs> {};

  srcDef = builtins.fromJSON (builtins.readFile ./nixpkgs.json);

  inherit (hostpkgs) fetchFromGitHub fetchpatch fetchurl;
in import (hostpkgs.stdenv.mkDerivation {
  name = "ofborg-nixpkgs-${builtins.substring 0 10 srcDef.rev}";
  phases = [ "unpackPhase" "patchPhase" "moveToOut" ];

  src = fetchFromGitHub {
    owner = "NixOS";
    repo = "nixpkgs-channels";
    inherit (srcDef) rev sha256;
  };

  patches = [
  ];

  moveToOut = ''
    root=$(pwd)
    cd ..
    mv "$root" $out
  '';
})
