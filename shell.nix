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
      ++ stdenv.lib.optional useNix1 nix;

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
      ++ stdenv.lib.optional useNix1 nix
      ++ stdenv.lib.optional stdenv.isDarwin pkgs.darwin.Security;

    HISTFILE = "${toString ./.}/.bash_hist";
    passthru.phpEnv = phpEnv;
  };


in rustEnv
