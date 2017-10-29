let
  pkgs = import <nixpkgs> {};

  inherit (pkgs) stdenv;

in stdenv.mkDerivation rec {
  name = "gh-event-forwarder";
  src = ./.;
  buildInputs = with pkgs; [
    php
    phpPackages.composer
  ];

  HISTFILE = "${src}/.bash_hist";
}
