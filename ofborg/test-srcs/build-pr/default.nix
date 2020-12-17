let
  builder = builtins.storePath <ofborg-test-bash>;
in
{
  stdenv = import <nix/fetchurl.nix> {
    url = "http://tarballs.nixos.org/stdenv-linux/x86_64/c5aabb0d603e2c1ea05f5a93b3be82437f5ebf31/bootstrap-tools.tar.xz";
    sha256 = "a5ce9c155ed09397614646c9717fc7cd94b1023d7b76b618d409e4fefd6e9d39";
  };

  success = derivation {
    name = "success";
    system = builtins.currentSystem;
    inherit builder;
    args = [ "-c" "echo hi; printf '1\n2\n3\n4\n'; echo ${toString builtins.currentTime} > $out" ];
  };

  failed = derivation {
    name = "failed";
    system = builtins.currentSystem;
    inherit builder;
    args = [ "-c" "echo hi; echo ${toString builtins.currentTime}" ];
  };

  sandbox-violation = derivation {
    name = "sandbox-violation";
    system = builtins.currentSystem;
    inherit builder;
    args = [ "-c" "echo hi; echo ${toString builtins.currentTime} > $out" ];
    src = ./../../src;
  };
}
