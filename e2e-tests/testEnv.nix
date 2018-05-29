{ config, pkgs, ... }:

{
  services.rabbitmq.enable = true;
  services.rabbitmq.plugins = [ "rabbitmq_management" ];
}
