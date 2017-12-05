let
  pkgs = import ./nix {};


  inherit (pkgs) stdenv;

  phpEnv = stdenv.mkDerivation rec {
    name = "gh-event-forwarder";
    src = null;
    buildInputs = with pkgs; [
      nix-prefetch-git
      php
      phpPackages.composer
      nix
      git
      php
      curl
      bash
    ];

    # HISTFILE = "${src}/.bash_hist";
  };

  rustEnv = stdenv.mkDerivation rec {
    name = "gh-event-forwarder";
    buildInputs = with pkgs; [
      nix-prefetch-git
      #php
      #phpPackages.composer
      rust.rustc
      rust.cargo
      carnix
      openssl.dev
      pkgconfig
    ];

    HISTFILE = "${toString ./.}/.bash_hist";
    passthru.phpEnv = phpEnv;
  };


in rustEnv
