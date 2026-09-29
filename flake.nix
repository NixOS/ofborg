{
  inputs = {
    nixpkgs.url = "https://channels.nixos.org/nixos-26.05/nixexprs.tar.zst";
  };

  outputs =
    {
      self,
      nixpkgs,
      ...
    }@inputs:
    let
      supportedSystems = [
        "aarch64-darwin"
        "x86_64-darwin"
        "x86_64-linux"
        "aarch64-linux"
      ];
      forAllSystems = f: nixpkgs.lib.genAttrs supportedSystems f;
    in
    {
      devShell = forAllSystems (system: inputs.self.devShells.${system}.default);
      devShells = forAllSystems (
        system:
        let
          pkgs = import nixpkgs {
            inherit system;
          };
        in
        {
          default = pkgs.mkShell {
            name = "gh-event-forwarder";
            nativeBuildInputs = with pkgs; [
              nix # so in --pure mode we actually find the "correct" nix
              bash
              nix-prefetch-git
              rustc
              cargo
              clippy
              rustfmt
              pkg-config
              git
            ];
            buildInputs =
              with pkgs;
              lib.optionals stdenv.isDarwin [
                darwin.Security
                libiconv
              ];

            postHook = ''
              checkPhase() (
                  cd "${builtins.toString ./.}/ofborg"
                  set -x
                  cargo fmt
                  git diff --exit-code
                  cargofmtexit=$?

                  cargo clippy
                  cargoclippyexit=$?

                  cargo build && cargo test
                  cargotestexit=$?

                  sum=$((cargofmtexit + cargoclippyexit + cargotestexit))
                  exit $sum
              )
            '';

            RUSTFLAGS = "-D warnings";
            RUST_BACKTRACE = "1";
            RUST_LOG = "ofborg=debug";
            NIX_PATH = "nixpkgs=${pkgs.path}";
          };
        }
      );

      packages = forAllSystems (
        system:
        let
          pkgs = import nixpkgs { inherit system; };

          # clippy fails to build on x86_64-darwin (nixpkgs missing
          # -headerpad_max_install_names, cf. rustfmt fix in nixpkgs#369349).
          # Skip the lint step there.
          runClippy = !(pkgs.stdenv.hostPlatform.isDarwin && pkgs.stdenv.hostPlatform.isx86_64);

          pkg = pkgs.rustPlatform.buildRustPackage {
            name = "ofborg";
            src = pkgs.nix-gitignore.gitignoreSource [ ] ./.;

            nativeBuildInputs =
              with pkgs;
              [
                pkg-config
              ]
              ++ pkgs.lib.optional runClippy pkgs.rustPackages.clippy;

            preBuild = pkgs.lib.optionalString runClippy ''
              cargo clippy
            '';

            doCheck = false; # Tests require access to a /nix/ and a nix daemon
            checkInputs = with pkgs; [
              nix
            ];

            cargoLock = {
              lockFile = ./Cargo.lock;
              outputHashes = {
                "hubcaps-0.6.2" = "sha256-Vl4wQIKQVRxkpQxL8fL9rndAN3TKLV4OjgnZOpT6HRo=";
                "hyperx-1.4.0" = "sha256-MW/KxxMYvj/DYVKrYa7rDKwrH6s8uQOCA0dR2W7GBeg=";
              };
            };
          };

        in
        {
          inherit pkg;

          ofborg.rs = pkgs.runCommand "ofborg-rs-symlink-compat" { src = pkg; } ''
            mkdir -p $out/bin
            for f in $(find $src -type f); do
              bn=$(basename "$f")
              ln -s "$f" "$out/bin/$bn"

              # Rust 1.n? or Cargo  starting outputting bins with dashes
              # instead of underscores ... breaking all the callers.
              if echo "$bn" | grep -q "-"; then
                ln -s "$f" "$out/bin/$(echo "$bn" | tr '-' '_')"
              fi
            done

            test -e $out/bin/builder
            test -e $out/bin/github_comment_filter
            test -e $out/bin/github_comment_poster
            test -e $out/bin/github_webhook_receiver
            test -e $out/bin/log_message_collector
            test -e $out/bin/evaluation_filter
          '';
        }
      );

      hydraJobs = {
        buildRs = forAllSystems (system: self.packages.${system}.ofborg.rs);
      };
    };
}
