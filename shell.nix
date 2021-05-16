{ pkgs ? import ./nix {
  overlays = [
    (import ./nix/overlay.nix)
  ];
} }:

let
  inherit (pkgs) stdenv lib;

  phpEnv = stdenv.mkDerivation rec {
    name = "gh-event-forwarder";
    src = null;
    buildInputs = with pkgs; [
      nix-prefetch-git
      php
      phpPackages.composer
      git
      php
      curl
      bash
    ];

    # HISTFILE = "${src}/.bash_hist";
  };

  rustEnv = stdenv.mkDerivation {
    name = "gh-event-forwarder";
    nativeBuildInputs = with pkgs; [
      nix # so in --pure mode we actually find the "correct" nix
      bash
      nix-prefetch-git
      rustPackages.cargo
      rustPackages.clippy
      rustPackages.rustfmt
      pkg-config
      git
    ];
    buildInputs = with pkgs; [
      openssl
    ]
      ++ lib.optional stdenv.isDarwin pkgs.darwin.Security;

    postHook = ''
      checkPhase() (
          cd "${builtins.toString ./.}/ofborg"
          set -x
          cargo fmt
          git diff --exit-code
          cargofmtexit=$?
  
          cargo clippy
          cargoclippyexit=$?
  

          cargo build && cargo test
          cargotestexit=$?

          sum=$((cargofmtexit + cargoclippyexit + cargotestexit))
          exit $sum
      )
    '';

    HISTFILE = "${toString ./.}/.bash_hist";
    RUSTFLAGS = "-D warnings";
    RUST_BACKTRACE = "1";
    RUST_LOG = "ofborg=debug";
    NIX_PATH = "nixpkgs=${pkgs.path}";
    passthru.phpEnv = phpEnv;
  };

in rustEnv
