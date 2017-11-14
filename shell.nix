let
  p = import <nixpkgs> {};
  pkgs = import (p.fetchFromGitHub {
    owner = "NixOS";
    repo = "nixpkgs-channels";
    rev = "cfafd6f5a819472911eaf2650b50a62f0c143e3e";
    sha256 = "10xgiyh4hbwwiy8qg70ma1f27nd717aflksk9fx3ci8bmxmqbkkn";
  }) {};


  inherit (pkgs) stdenv;

  phpEnv = stdenv.mkDerivation rec {
    name = "gh-event-forwarder";
    src = null;
    buildInputs = with pkgs; [
      php
      phpPackages.composer
      nix
      git
      php
      curl
      bash
    ];

    HISTFILE = "${src}/.bash_hist";
    passthru.rustEnv = rustEnv;
  };

  rustEnv = stdenv.mkDerivation rec {
    name = "gh-event-forwarder";
    buildInputs = with pkgs; [
      php
      phpPackages.composer
      rust.rustc
      rust.cargo
      openssl.dev
      pkgconfig
    ];

    HISTFILE = "${toString ./.}/.bash_hist";
  };


in phpEnv
