{ pkgs ? import ./nix {
    overlays = [ (import ./nix/overlay.nix) ];
  }
}:

let
  inherit (pkgs) lib;

  pkg = pkgs.rustPlatform.buildRustPackage {
    name = "ofborg";
    src = pkgs.nix-gitignore.gitignoreSource [ ] ./.;

    nativeBuildInputs = with pkgs; [
      pkgconfig
      pkgs.rustPackages.clippy
    ];

    buildInputs = with pkgs; [
      openssl
    ] ++ lib.optionals pkgs.stdenv.isDarwin (with pkgs; [
      darwin.apple_sdk.frameworks.Security
      darwin.apple_sdk.frameworks.CoreFoundation
    ]);

    preBuild = ''
      cargo clippy
    '';

    doCheck = false; # Tests require access to a /nix/ and a nix daemon
    checkInputs = with pkgs; [
      nix
    ];

    cargoLock = {
      lockFile = ./Cargo.lock;
      outputHashes = {
        "hubcaps-0.3.16" = "sha256-/BFXGccu27K8heK4IL7JnS/U7zatTk9wRybhtxppADM=";
      };
    };
  };
in

{
  inherit pkg;

  ofborg.rs = pkgs.runCommand "ofborg-rs-symlink-compat" { src = pkg; } ''
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
  '';

  ofborg.php = import ./php { inherit pkgs; };
}
