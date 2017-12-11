{ checkMeta }:
let
  lib = import ./lib;
  hydraJobs = import ./pkgs/top-level/release.nix
    # Compromise: accuracy vs. resources needed for evaluation.
    {
      supportedSystems = [
        # Not ready to evaluate these archs, see #32365
        # "aarch64-linux"
        # "i686-linux"
        "x86_64-linux"
        "x86_64-darwin"
      ];
      nixpkgsArgs = {
        config = {
          allowBroken = true;
          allowUnfree = true;
          allowInsecurePredicate = x: true;
          checkMeta = false; # checkMeta; see #32365

          # See https://github.com/NixOS/nixpkgs/pull/32365
          handleEvalIssue = reason: errormsg:
            if reason == "unknown-meta"
              then (builtins.trace (abort errormsg) true)
              else (builtins.trace errormsg true);

          inHydra = true;
        };
      };
    };
  recurseIntoAttrs = attrs: attrs // { recurseForDerivations = true; };

  # hydraJobs leaves recurseForDerivations as empty attrmaps;
  # that would break nix-env and we also need to recurse everywhere.
  tweak = lib.mapAttrs
    (name: val:
      if name == "recurseForDerivations" then true
      else if lib.isAttrs val && val.type or null != "derivation"
              then recurseIntoAttrs (tweak val)
      else val
    );

  # Some of these contain explicit references to platform(s) we want to avoid;
  # some even (transitively) depend on ~/.nixpkgs/config.nix (!)
  blacklist = [
    "tarball" "metrics" "manual"
    "darwin-tested" "unstable" "stdenvBootstrapTools"
    "moduleSystem" "lib-tests" # these just confuse the output
  ];

in
  tweak (builtins.removeAttrs hydraJobs blacklist)
