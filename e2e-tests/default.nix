import <nixpkgs/nixos/tests/make-test.nix> ({ pkgs, ...}: rec {
  
  testData = pkgs.runCommand "ofborg-gh-test" { src = ./.; } ''
                mkdir -p $out
                cp -r $src/fixtures $out
                cp $src/config.test.json $out
                cp $src/testGhPR.sh $out
             '';
  machine =
    { config, pkgs, ... }:
    { 
      # Build local ofborg version
      nixpkgs.config = {
          packageOverrides = pkgs: with pkgs; {
            ofborgGit = (import ../default.nix {}).ofborg.rs;
            ofborgGhTest = testData;
          };
      };

      environment.systemPackages = [ pkgs.ofborgGit pkgs.ofborgGhTest pkgs.wget pkgs.python ];
      services.rabbitmq = {
        enable = true;
        plugins = [ "rabbitmq_management" ];
      };
    };

  

  testScript = 
    ''
      startAll;
      $machine->waitForUnit("rabbitmq");
      $machine->execute("${testData}/testGhPR.sh ${testData}/config.test.json ${testData}/fixtures");
    '';
})
