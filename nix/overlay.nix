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
  };
})

