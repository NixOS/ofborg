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
  ofborg.rs = stripDeps (pkgs.callPackage ./nix/ofborg-carnix.nix {}).ofborg_0_1_0;
  ircbot = stripDeps (pkgs.callPackage ./nix/ircbot-carnix.nix {}).ircbot_0_1_0;

  # /nix/store/a4xfsgh5bwh5w4m9y1j40ry4dq892gl4-ofborg/
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
