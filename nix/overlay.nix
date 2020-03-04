(self: super: {
  defaultCrateOverrides = super.defaultCrateOverrides // {
    openssl-sys = attrs: {
      buildInputs = [ self.openssl_1_0_2 ];
      nativeBuildInputs = [ self.pkgconfig ];
    };
    openssl = attrs: {
      DEP_OPENSSL_VERSION = "102";
    };
  };
})

