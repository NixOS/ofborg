{ pkgs ? import ./nix {
  overlays = [
    (import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz))
  ];
}, useNix1 ? false }:

let
  # A random Nixpkgs revision *before* the default glibc
  # was switched to version 2.27.x.
  oldpkgsSrc = pkgs.fetchFromGitHub {
    owner = "nixos";
    repo = "nixpkgs";
    rev = "0252e6ca31c98182e841df494e6c9c4fb022c676";
    sha256 = "1sr5a11sb26rgs1hmlwv5bxynw2pl5w4h5ic0qv3p2ppcpmxwykz";
  };

  oldpkgs = import oldpkgsSrc {};

  inherit (pkgs) stdenv;

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
    ]
      ++ stdenv.lib.optional useNix1 oldpkgs.nix1;

    # HISTFILE = "${src}/.bash_hist";
  };

  mozilla-rust-overlay = stdenv.mkDerivation (rec {
    name = "mozilla-rust-overlay";
    buildInputs = with pkgs; [
      latest.rustChannels.stable.rust
      git
      pkgconfig
      openssl.dev
    ]
      ++ stdenv.lib.optional stdenv.isDarwin pkgs.darwin.Security;

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
  }
  // stdenv.lib.optionalAttrs stdenv.isLinux {
    LOCALE_ARCHIVE_2_21 = "${oldpkgs.glibcLocales}/lib/locale/locale-archive";
    LOCALE_ARCHIVE_2_27 = "${pkgs.glibcLocales}/lib/locale/locale-archive";
  });

  rustEnv = stdenv.mkDerivation (rec {
    name = "gh-event-forwarder";
    buildInputs = with pkgs; [
      bash
      nix-prefetch-git
      latest.rustChannels.stable.rust
      #rustfmt
      #carnix
      openssl.dev
      pkgconfig
      git
    ]
      ++ stdenv.lib.optional useNix1 oldpkgs.nix1
      ++ stdenv.lib.optional stdenv.isDarwin pkgs.darwin.Security;

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
  }
  // stdenv.lib.optionalAttrs stdenv.isLinux {
    LOCALE_ARCHIVE_2_21 = "${oldpkgs.glibcLocales}/lib/locale/locale-archive";
    LOCALE_ARCHIVE_2_27 = "${pkgs.glibcLocales}/lib/locale/locale-archive";
  });

in rustEnv
