{ pkgs ? import ./nix {
  overlays = [
    (import ./nix/overlay.nix)
    (import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz))
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

  mozilla-rust-overlay = stdenv.mkDerivation {
    name = "mozilla-rust-overlay";
    buildInputs = with pkgs; [
      latest.rustChannels.stable.rust
      git
      pkg-config
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

        sum=$((cargofmtexit + cargoclippyexit))
        exit $sum
      )
    '';

    RUSTFLAGS = "-D warnings";
    RUST_BACKTRACE = "1";
    NIX_PATH = "nixpkgs=${pkgs.path}";
  };

  rustEnv = stdenv.mkDerivation {
    name = "gh-event-forwarder";
    buildInputs = with pkgs; [
      bash
      nix-prefetch-git
      latest.rustChannels.stable.rust
      #rustfmt
      openssl
      pkg-config
      git
    ]
      ++ lib.optional stdenv.isDarwin pkgs.darwin.Security;

    postHook = ''
      checkPhase() {
          ( cd "${builtins.toString ./.}/ofborg" && cargo build && cargo test)
      }
    '';

    HISTFILE = "${toString ./.}/.bash_hist";
    RUSTFLAGS = "-D warnings";
    RUST_BACKTRACE = "1";
    RUST_LOG = "ofborg=debug";
    NIX_PATH = "nixpkgs=${pkgs.path}";
    passthru.phpEnv = phpEnv;
    passthru.mozilla-rust-overlay = mozilla-rust-overlay;
  };

in rustEnv
