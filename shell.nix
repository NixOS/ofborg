let
  pkgs = import <nixpkgs> {};

  inherit (pkgs) stdenv;

  phpEnv = stdenv.mkDerivation rec {
    name = "gh-event-forwarder";
    src = ./.;
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
    src = ./.;
    buildInputs = with pkgs; [
      php
      phpPackages.composer
      rust.rustc
      rust.cargo
      openssl
    ];

    HISTFILE = "${src}/.bash_hist";
  };


in phpEnv
