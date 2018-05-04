{ pkgs ? import ./nix {}, useNix1 ? true }:

let
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
      ++ stdenv.lib.optional useNix1 nix1;

    # HISTFILE = "${src}/.bash_hist";
  };

  rustEnv = stdenv.mkDerivation rec {
    name = "gh-event-forwarder";
    buildInputs = with pkgs; [
      nix-prefetch-git
      rust.rustc
      rust.cargo
      rustfmt
      carnix
      openssl.dev
      pkgconfig
      git
    ]
      ++ stdenv.lib.optional useNix1 nix1
      ++ stdenv.lib.optional stdenv.isDarwin pkgs.darwin.Security;

    postHook = ''
      checkPhase() {
          ( cd "${builtins.toString ./.}/ofborg" && cargo test --lib )
      }

      export NIX_REMOTE=daemon
    '';

    HISTFILE = "${toString ./.}/.bash_hist";
    RUSTFLAGS = "-D warnings";

    passthru.phpEnv = phpEnv;
  };


in rustEnv
