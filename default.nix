{ pkgs ? import ./nix {} }:

let
  ofborgOverrides = {
    crateOverrides = pkgs.defaultCrateOverrides // {
      ofborg = attrs: {
        buildInputs = pkgs.lib.optional pkgs.stdenv.isDarwin
          pkgs.darwin.apple_sdk.frameworks.Security;
      };
    };
  };

  drv = (pkgs.callPackage ./ofborg/Cargo.nix {
    cratesIO = pkgs.callPackage ./ofborg/crates-io.nix {};
  }).ofborg {};

  src = stripDeps (drv.override ofborgOverrides);

  stripDeps = pkg: pkgs.runCommand "${pkg.name}-deps-stripped" {}
  ''
    cp -r ${pkg} $out
    chmod -R a+w $out
    rm -rf $out/lib
    find $out/bin -name '*.d' -delete
    chmod -R a-w $out
  '';
in

{
  ofborg.rs = pkgs.runCommand "ofborg-rs-symlink-compat" { inherit src; } ''
    mkdir -p $out/bin
    for f in $(find $src -type f); do
      bn=$(basename "$f")
      ln -s "$f" "$out/bin/$bn"

      # Rust 1.n? or Cargo  starting outputting bins with dashes
      # instead of underscores ... breaking all the callers.
      if echo "$bn" | grep -q "-"; then
        ln -s "$f" "$out/bin/$(echo "$bn" | tr '-' '_')"
      fi
    done

    test -e $out/bin/builder
    test -e $out/bin/github_comment_filter
    test -e $out/bin/github_comment_poster
    test -e $out/bin/log_message_collector
    test -e $out/bin/evaluation_filter

    # Verify that the outpath contains the version number matching the
    # Cargo.toml
    if ! grep -q 'version = "${drv.crateVersion}"' ${./ofborg/Cargo.toml}; then
      cat <<EOF





    Build failed because you bumped the Cargo
    version without regenerating the carnix
    file.

    Run:



        nix-shell --run ./nix/update-carnix.sh


    and commit those changes.


    EOF
    fi
  '';

  ofborg.php = import ./php { inherit pkgs; };
}
