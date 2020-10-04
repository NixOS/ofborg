{composerEnv, fetchurl, fetchgit ? null, fetchhg ? null, fetchsvn ? null, noDev ? false}:

let
  packages = {
    "php-amqplib/php-amqplib" = {
      targetDir = "";
      src = composerEnv.buildZipPackage {
        name = "php-amqplib-php-amqplib-0eaaa9d5d45335f4342f69603288883388c2fe21";
        src = fetchurl {
          url = https://api.github.com/repos/php-amqplib/php-amqplib/zipball/0eaaa9d5d45335f4342f69603288883388c2fe21;
          sha256 = "0dpjy33rspmpdflhwjqb9iass8kxzbl3nj8nc3vgn9hczgaxqlfs";
        };
      };
    };
    "phpseclib/phpseclib" = {
      targetDir = "";
      src = composerEnv.buildZipPackage {
        name = "phpseclib-phpseclib-497856a8d997f640b4a516062f84228a772a48a8";
        src = fetchurl {
          url = https://api.github.com/repos/phpseclib/phpseclib/zipball/497856a8d997f640b4a516062f84228a772a48a8;
          sha256 = "061kgl49f1zc5vdfrlmq6m1qgvqrh7jvlldfhfxya44y2vwriz1p";
        };
      };
    };
    "svanderburg/composer2nix" = {
      targetDir = "";
      src = composerEnv.buildZipPackage {
        name = "svanderburg-composer2nix-57cecaf5d9d667b47415bb7c1d1f5154be7c759e";
        src = fetchurl {
          url = https://api.github.com/repos/svanderburg/composer2nix/zipball/57cecaf5d9d667b47415bb7c1d1f5154be7c759e;
          sha256 = "0s6fjwaf2dwzf9h83dms5wg8s3a1kcy5nmdnn7wy1ykqi3mhp61m";
        };
      };
    };
    "svanderburg/pndp" = {
      targetDir = "";
      src = composerEnv.buildZipPackage {
        name = "svanderburg-pndp-4bfe9c4120c23354ab8dc295957dc3009a39bff0";
        src = fetchurl {
          url = https://api.github.com/repos/svanderburg/pndp/zipball/4bfe9c4120c23354ab8dc295957dc3009a39bff0;
          sha256 = "0n2vwpwshv16bhb7a6j95m664zh4lpfa7dqmcyhmn89nxpgvg91y";
        };
      };
    };
  };
  devPackages = {};
in
composerEnv.buildPackage {
  inherit packages devPackages noDev;
  name = "ofborg-webhook";
  src = ./.;
  executable = false;
  symlinkDependencies = false;
  meta = {};
}