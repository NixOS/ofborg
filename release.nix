{ nixpkgs ? ./nix
, supportedSystems ? [ "x86_64-linux" "x86_64-darwin" "aarch64-linux" ]
}:
let
  pkgs = import nixpkgs {};
  inherit (pkgs) lib;

  # An attrset of borgpkgs per supportedSystem:
  #
  # {
  #   "x86_64-linux" = ...
  #   "x86_64-darwin" = ...
  # }
  borgpkgs-per-arch = builtins.foldl'
    (collector: system:
      collector // {
        "${system}" = import ./. {
          pkgs = import nixpkgs { inherit system; };
        };
      }
    )
    {} supportedSystems;

  attrForSystem = system: attrpath:
    if borgpkgs-per-arch ? "${system}"
    then (
      let
        borgpkgs = borgpkgs-per-arch."${system}";
      in if lib.hasAttrByPath attrpath borgpkgs
        then lib.setAttrByPath
          (attrpath ++ [system])
          (lib.attrByPath attrpath "bogus" borgpkgs)
        else throw "Failed to find ${toString attrpath} for ${system} in borgpkgs"
    )
    else throw "No such system ${system}";

  attrsForAllSystems = path:
    builtins.foldl'
      (collector: system:
        lib.recursiveUpdate collector (attrForSystem system path)
      )
      {}
      supportedSystems;

  merge = attrsets:
    builtins.foldl'
      (collector: set: lib.recursiveUpdate set collector)
      {}
      attrsets;

  x8664LinuxOnly = path:
     (attrForSystem "x86_64-linux" path);

  jobs = merge [
    (attrsForAllSystems [ "ofborg" "rs" ])

    (x8664LinuxOnly [ "ofborg" "php" ])
  ];
in jobs // {
  release = pkgs.releaseTools.aggregate {
    name = "release";
    meta.description = "Release-critical builds for OfBorg infrastructure";
    constituents = [
      jobs.ofborg.rs.x86_64-linux
      jobs.ofborg.rs.x86_64-darwin
      # jobs.ofborg.rs.aarch64-linux
      jobs.ofborg.php.x86_64-linux
    ];
  };
}
