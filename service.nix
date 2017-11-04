{ pkgs, ... }:
let
  src = ./.;
in {
  users.users.gc-of-borg = {
    description = "GC Of Borg Workers";
    home = "/var/lib/gc-of-borg";
    createHome = true;
    group = "gc-of-borg";
    uid = 402;
  };
  users.groups.gc-of-borg.gid = 402;

  systemd.services = {
    "grahamcofborg-builder" = {
      enable = true;
      after = [ "network.target" "network-online.target" ];
      wants = [ "network-online.target" ];
      wantedBy = [ "multi-user.target" ];

      path = with pkgs; [
        nix
        git
        php
        curl
        bash
      ];

      serviceConfig = {
        User = "gc-of-borg";
        Group = "gc-of-borg";
        PrivateTmp = true;
        WorkingDirectory = "/var/lib/gc-of-borg";
        Restart = "always";
      };

      preStart = ''
        mkdir -p ./.nix-test
      '';

      script = ''
        export HOME=/var/lib/gc-of-borg;
        export NIX_REMOTE=daemon;
        export NIX_PATH=nixpkgs=/run/current-system/nixpkgs;
        git config --global user.email "graham+cofborg@grahamc.com"
        git config --global user.name "GrahamCOfBorg"
        php ${src}/builder.php
      '';
    };
  };
}
