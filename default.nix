{ pkgs ? import ./nix {
    overlays = [ (import ./nix/overlay.nix) ];
  }
, ofborgCrates ? import ./Cargo.nix {
    inherit pkgs;
    release = pkgs.stdenv.isDarwin;
  }
}:

let
  inherit (ofborgCrates.workspaceMembers) ofborg-simple-build ofborg;
in

{
  inherit ofborgCrates;

  ofborg.simple-build = ofborg-simple-build.build;

  ofborg.rs = pkgs.runCommand "ofborg-rs-symlink-compat" { src = ofborg.build; } ''
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
    if ! grep -q 'version = "${ofborg.build.crateVersion}"' ${./ofborg/Cargo.toml}; then
      cat <<EOF





    Build failed because you bumped the Cargo
    version without regenerating the Cargo.nix.

    Run:



        nix-shell --run ./nix/update-crates.sh


    and commit those changes.


    EOF
    fi
  '';

  ofborg.php = import ./php { inherit pkgs; };
}
