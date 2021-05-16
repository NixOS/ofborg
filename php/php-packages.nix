{composerEnv, fetchurl, fetchgit ? null, fetchhg ? null, fetchsvn ? null, noDev ? false}:

let
  packages = {
    "paragonie/constant_time_encoding" = {
      targetDir = "";
      src = composerEnv.buildZipPackage {
        name = "paragonie-constant_time_encoding-f34c2b11eb9d2c9318e13540a1dbc2a3afbd939c";
        src = fetchurl {
          url = "https://api.github.com/repos/paragonie/constant_time_encoding/zipball/f34c2b11eb9d2c9318e13540a1dbc2a3afbd939c";
          sha256 = "1r1xj3j7s5mskw5gh3ars4dfhvcn7d252gdqgpif80026kj5fvrp";
        };
      };
    };
    "paragonie/random_compat" = {
      targetDir = "";
      src = composerEnv.buildZipPackage {
        name = "paragonie-random_compat-996434e5492cb4c3edcb9168db6fbb1359ef965a";
        src = fetchurl {
          url = "https://api.github.com/repos/paragonie/random_compat/zipball/996434e5492cb4c3edcb9168db6fbb1359ef965a";
          sha256 = "0ky7lal59dihf969r1k3pb96ql8zzdc5062jdbg69j6rj0scgkyx";
        };
      };
    };
    "php-amqplib/php-amqplib" = {
      targetDir = "";
      src = composerEnv.buildZipPackage {
        name = "php-amqplib-php-amqplib-c0a8eade209b7e43d6a405303d8de716dfd02749";
        src = fetchurl {
          url = "https://api.github.com/repos/php-amqplib/php-amqplib/zipball/c0a8eade209b7e43d6a405303d8de716dfd02749";
          sha256 = "1lck5hpraxi4w3831lfzz6l7c82118b56aghdz0dr4r92366j3xf";
        };
      };
    };
    "phpseclib/phpseclib" = {
      targetDir = "";
      src = composerEnv.buildZipPackage {
        name = "phpseclib-phpseclib-d9615a6fb970d9933866ca8b4036ec3407b020b6";
        src = fetchurl {
          url = "https://api.github.com/repos/phpseclib/phpseclib/zipball/d9615a6fb970d9933866ca8b4036ec3407b020b6";
          sha256 = "1xyymb11qdx8rn6c9bx9js8yq405bmjq8ar8phvpji9a3fdd6z4v";
        };
      };
    };
    "svanderburg/composer2nix" = {
      targetDir = "";
      src = composerEnv.buildZipPackage {
        name = "svanderburg-composer2nix-9983c6fafb277f6b305edf232c2690d6d28ec092";
        src = fetchurl {
          url = "https://api.github.com/repos/svanderburg/composer2nix/zipball/9983c6fafb277f6b305edf232c2690d6d28ec092";
          sha256 = "1s1gv2b4y9pjv56mif8fgch56sssdmrcwb1gk3gksxc0jq2w2zbv";
        };
      };
    };
    "svanderburg/pndp" = {
      targetDir = "";
      src = composerEnv.buildZipPackage {
        name = "svanderburg-pndp-4bfe9c4120c23354ab8dc295957dc3009a39bff0";
        src = fetchurl {
          url = "https://api.github.com/repos/svanderburg/pndp/zipball/4bfe9c4120c23354ab8dc295957dc3009a39bff0";
          sha256 = "0n2vwpwshv16bhb7a6j95m664zh4lpfa7dqmcyhmn89nxpgvg91y";
        };
      };
    };
  };
  devPackages = {};
in
composerEnv.buildPackage {
  inherit packages devPackages noDev;
  name = "nixos-ofborg-webhook";
  src = ./.;
  executable = false;
  symlinkDependencies = false;
  meta = {};
}
