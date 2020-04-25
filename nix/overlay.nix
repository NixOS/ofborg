(self: super:
{
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

