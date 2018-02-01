{ pkgs ? import ./nix {}
}:
let
  stripDeps = pkg: pkgs.runCommand "${pkg.name}-deps-stripped" {}
  ''
    cp -r ${pkg} $out
    chmod -R a+w $out
    rm -rf $out/lib
    find $out/bin -name '*.d' -delete
    chmod -R a-w $out
  '';
in {
  ofborg.rs = let
      build = (pkgs.callPackage ./nix/ofborg-carnix.nix {})
        .ofborg_0_1_1.override {
          crateOverrides = pkgs.defaultCrateOverrides //
          {
            ofborg = attrs: {
              buildInputs =
                  pkgs.lib.optionals
                    pkgs.stdenv.isDarwin
                    [ pkgs.darwin.apple_sdk.frameworks.Security ];
            };
          };
        };
    in stripDeps build;
  ircbot = stripDeps (pkgs.callPackage ./nix/ircbot-carnix.nix {}).ircbot_0_1_0;

  ofborg.php = pkgs.runCommand
    "ofborg"
    {
      src = builtins.filterSource
        (path: type: !(
             (type == "symlink" && baseNameOf path == "result")
          || (type == "directory" && baseNameOf path == ".git")
        ))
        ./php;
    }
    ''
      cp -r $src ./ofborg
      chmod -R u+w ./ofborg
      cd ofborg
      ls -la
      cd ..
      mv ofborg $out
    '';
}
