(import
  (fetchTarball {
    url = "https://github.com/edolstra/flake-compat/archive/0f9255e01c2351cc7d116c072cb317785dd33b33.tar.gz";
    sha256 = "0m9grvfsbwmvgwaxvdzv6cmyvjnlww004gfxjvcl806ndqaxzy4j";
  })
  { src = ./.; }).defaultNix.packages.${builtins.currentSystem}
