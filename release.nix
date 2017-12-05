{ nixpkgs ? ./nix
, supportedSystems ? [ "x86_64-linux" "x86_64-darwin" ]
}:
let
  pkgs = import nixpkgs {};
  inherit (pkgs) lib;

  ofborgpkgs = lib.genAttrs supportedSystems (system:
    (import ./default.nix { pkgs = import nixpkgs { inherit system; }; })
  );
in ofborgpkgs
