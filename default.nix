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
}
