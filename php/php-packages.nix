{composerEnv, fetchurl, fetchgit ? null, fetchhg ? null, fetchsvn ? null, noDev ? false}:

let
  packages = {
    "php-amqplib/php-amqplib" = {
      targetDir = "";
      src = composerEnv.buildZipPackage {
        name = "php-amqplib-php-amqplib-dfd3694a86f1a7394d3693485259d4074a6ec79b";
        src = fetchurl {
          url = https://api.github.com/repos/php-amqplib/php-amqplib/zipball/dfd3694a86f1a7394d3693485259d4074a6ec79b;
          sha256 = "1dlxgdnhy8xyx8xbp1glc7igksvsqyc3yaq76irhy09djij013ip";
        };
      };
    };
    "svanderburg/composer2nix" = {
      targetDir = "";
      src = composerEnv.buildZipPackage {
        name = "svanderburg-composer2nix-2fb157acaf0ecbe34436195c694637396f7258a6";
        src = fetchurl {
          url = https://api.github.com/repos/svanderburg/composer2nix/zipball/2fb157acaf0ecbe34436195c694637396f7258a6;
          sha256 = "01i3kxgx7pcmxafclp8ib08nib1xh6nvr5sbl6y38rw19xhnwa0m";
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