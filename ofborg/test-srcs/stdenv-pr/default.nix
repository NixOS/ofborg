let
  builder = builtins.storePath <ofborg-test-bash>;
in
{
  stdenv = import <nix/fetchurl.nix> {
    url = "http://tarballs.nixos.org/stdenv-linux/x86_64/c5aabb0d603e2c1ea05f5a93b3be82437f5ebf31/bootstrap-tools.tar.xz";
    sha256 = "0000000000000000000000000000000000000000000000000000000000000000";
  };

  success = derivation {
    name = "success";
    system = builtins.currentSystem;
    inherit builder;
    args = [ "-c" "echo hi; printf '1\n2\n3\n4\n'; echo ${toString builtins.currentTime} > $out" ];
  };
}
