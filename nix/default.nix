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
    (fetchurl {
      # As of 2017-22-29 21:30:00 EST:
      # The URL is https://github.com/NixOS/nixpkgs/pull/31150 but
      # with master merged in so it applies cleanly.
      url = "https://github.com/NixOS/nixpkgs/compare/master...grahamc:P-E-Meunier-mkRustCrate-rebased.patch";
      name = "mkRustCrate-rebased.patch";
      sha256 = "09808bwcd3d5j2mnlzxq60g53rd07af7in1yfmpxvmi30sw4hi9b";
    })

  ];

  moveToOut = ''
    root=$(pwd)
    cd ..
    mv "$root" $out
  '';
})
