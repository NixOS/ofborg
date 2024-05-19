{composerEnv, fetchurl, fetchgit ? null, fetchhg ? null, fetchsvn ? null, noDev ? false}:

let
  packages = {
    "paragonie/constant_time_encoding" = {
      targetDir = "";
      src = composerEnv.buildZipPackage {
        name = "paragonie-constant_time_encoding-52a0d99e69f56b9ec27ace92ba56897fe6993105";
        src = fetchurl {
          url = "https://api.github.com/repos/paragonie/constant_time_encoding/zipball/52a0d99e69f56b9ec27ace92ba56897fe6993105";
          sha256 = "1ja5b3fm5v665igrd37vs28zdipbh1xgh57lil2iaggvh1b8kh4x";
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
        name = "php-amqplib-php-amqplib-cb514530ce45a6d2f636be5196010c47c3bcf6e0";
        src = fetchurl {
          url = "https://api.github.com/repos/php-amqplib/php-amqplib/zipball/cb514530ce45a6d2f636be5196010c47c3bcf6e0";
          sha256 = "0mjca0m9960m8xgi22azwk8v1lgg8yznxscw10sqfzgp4wj4sfv0";
        };
      };
    };
    "phpseclib/phpseclib" = {
      targetDir = "";
      src = composerEnv.buildZipPackage {
        name = "phpseclib-phpseclib-cfa2013d0f68c062055180dd4328cc8b9d1f30b8";
        src = fetchurl {
          url = "https://api.github.com/repos/phpseclib/phpseclib/zipball/cfa2013d0f68c062055180dd4328cc8b9d1f30b8";
          sha256 = "1wgzy4fbj565czpn9xasr8lnd9ilh1x3bsalrpx5bskvqr4zspgj";
        };
      };
    };
    "svanderburg/composer2nix" = {
      targetDir = "";
      src = composerEnv.buildZipPackage {
        name = "svanderburg-composer2nix-299caca4aac42d7639a42eb4dde951c010f6e91c";
        src = fetchurl {
          url = "https://api.github.com/repos/svanderburg/composer2nix/zipball/299caca4aac42d7639a42eb4dde951c010f6e91c";
          sha256 = "0vb7q4za6z89azz4c5v7hgcv9gblcpk7hffl6va7q5f27fyyhwy0";
        };
      };
    };
    "svanderburg/pndp" = {
      targetDir = "";
      src = composerEnv.buildZipPackage {
        name = "svanderburg-pndp-bc795b341d95c24bb577e0d7a4a37fde98b1cce8";
        src = fetchurl {
          url = "https://api.github.com/repos/svanderburg/pndp/zipball/bc795b341d95c24bb577e0d7a4a37fde98b1cce8";
          sha256 = "1y46wsccjwdkvs1c1bklwbp7crsg0axyr7ncdibbny1sr54xb24i";
        };
      };
    };
  };
  devPackages = {};
in
composerEnv.buildPackage {
  inherit packages devPackages noDev;
  name = "nixos-ofborg-webhook";
  src = composerEnv.filterSrc ./.;
  executable = false;
  symlinkDependencies = false;
  meta = {};
}
