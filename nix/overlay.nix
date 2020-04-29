(self: super:
{
  carnix = super.carnix.overrideDerivation (drv: {
    patches = super.patches or [] ++ [
      (super.fetchurl {
        name = "carnix-cfg.patch";
        url = "https://gist.githubusercontent.com/LnL7/27a567cd2b3162a21cbd0499c6fa0f71/raw/32d3055b6ce105c2c64e8cdfe0204d6c90f6d214/carnix-cfg.patch";
        sha256 = "1nc5dlxqrhgh989dfzhjqw46hk3aii0rcz1qr3cvqbrwc7wzcj6w";
      })
      (super.fetchurl {
        name = "carnix-workspaces.patch";
        url = "https://gist.githubusercontent.com/LnL7/27a567cd2b3162a21cbd0499c6fa0f71/raw/d6395cfc06dff2a3555b0068e477274f9560fbae/carnix-workspace.patch";
        sha256 = "1kvfing0s968pknsrpc98yjii8idsqmy00dsvwkyfbqx9frn7kjg";
      })
    ];
  });

  defaultCrateOverrides = super.defaultCrateOverrides // {
    ofborg = attrs: {
      buildInputs = with self.darwin.apple_sdk.frameworks;
        super.lib.optional super.stdenv.isDarwin Security;
    };
    ofborg-simple-build = attrs: {
      buildInputs = with self.darwin.apple_sdk.frameworks;
        super.lib.optional super.stdenv.isDarwin Security;
    };
    openssl-sys = attrs: {
      buildInputs = [ self.openssl_1_0_2 ];
      nativeBuildInputs = [ self.pkgconfig ];
    };
    openssl = attrs: {
      DEP_OPENSSL_VERSION = "102";
    };
  };
})

