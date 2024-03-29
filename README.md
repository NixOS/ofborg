# ofborg

## Guidelines

1. Review the code of all PRs before triggering the bot on them.
2. Be gentle; try not to run mass rebuilds or massive builds (like Chromium) on
   it.

## Automatic Building

All users will have their PRs automatically trigger builds if their commits
follow the well-defined format of Nixpkgs. Specifically: prefixing the commit
title with the package attribute. This includes package bumps as well as other
changes.

Example commit titles and the builds they will start:

| Message                                                               | Automatic Build                                          |
|-----------------------------------------------------------------------|----------------------------------------------------------|
| `vim: 1.0.0 -> 2.0.0`                                                 | `vim`                                                    |
| `vagrant: Fix dependencies for version 2.0.2 `                        | `vagrant`                                                |
| `python36Packages.requests,python27Packages.requests: 1.0.0 -> 2.0.0` | `python36Packages.requests`, `python27Packages.requests` |
| `python{27,310}Packages.requests: 1.0.0 -> 2.0.0`                        | `python27Packages.requests`, `python310Packages.requests`   |

When opening a PR with multiple commits, ofborg creates a single build job for
all detected packages. If multiple commits get pushed to a PR one-by-one, each
detected package will get a separate build job.

If the title of a PR begins with `WIP:`, contains `[WIP]` anywhere, or has the
`2.status: work-in-progress` label, its packages are not built automatically.
**Note**: Marking a PR as a draft does not prevent automatic builds.

## Commands

The comment parser is line-based, so commentary can be interwoven with
instructions for ofborg.

1. To trigger the bot, the line _must_ start with `@ofborg` (case insensitive).
   * **Note**: GitHub will not suggest `@ofborg` to you, but it will work all
     the same. When in doubt, preview your comment and verify that `@ofborg`
     links to https://github.com/ofborg/.
2. To use multiple commands, separate them with whitespace. For examples, see
   the "[Multiple Commands](#multiple-commands)" section.

### test

```
@ofborg test list of tests
```

This will run `nix-build ./default.nix -A nixosTests.list -A nixosTests.of -A
nixosTests.tests` from the root of the Nixpkgs checkout.

Tests will run on all allowed machines. For more information, see the "[Trusted
Users](#trusted-users)" section.

### eval

```
@ofborg eval
```

See "[How does ofborg call
`nix-instantiate`?](#how-does-ofborg-call-nix-instantiate)" for what command(s)
this will run.

**Note**: Every PR automatically evaluates both upon creation and when the
commits change. There is no reason to run eval on a PR unless the evaluation
failed for weird reasons or master was previously broken.

### build

```
@ofborg build list of attrs
```

This will run `nix-build ./default.nix -A list -A of -A attrs` from the root of
the Nixpkgs checkout (see also "[How does ofborg call
`nix-build`?](#how-does-ofborg-call-nix-build)").

Builds will run on all allowed machines. For more information, see the "[Trusted
Users](#trusted-users)" section.

## Multiple Commands

You can use multiple commands in a variety ways. Here are some valid
combinations:

*
    ```
    @ofborg build list of attrs
    @ofborg eval
    ```

*
    ```
    @ofborg build list of attrs @ofborg eval
    ```

*
    ```
    looks good to me!
    @ofborg eval
    @ofborg build list of attrs
    ```

*
    ```
    @ofborg eval
    @ofborg build list of attrs
    looks good to me!
    ```

*
    ```
    @ofborg build list of attrs
    @ofborg test list of attrs
    ```

* This will build `list`, `of`, `attrs`, `looks`, `good`, `to`, and `me!` (which is probably not what you want):
    ```
    @ofborg build list of attrs looks good to me!
    ```

## Trusted Users (Currently Disabled)

> **NOTE:** The Trusted Users functionality is currently disabled, as the
> current darwin builder is reset very frequently. This means that _all_ users
> will have their PRs build on the darwin machine.

Trusted users have their builds and tests executed on _all_ available platforms,
including those without good sandboxing. Because this exposes the host to a
higher risk of security issues, the trusted users list consists of only
well-known, trusted members of the community.

At the time of writing, trusted users have their builds and tests run on these
platforms:

 - `x86_64-linux`
 - `aarch64-linux`
 - `x86_64-darwin`
 - `aarch64-darwin`

See [`config.public.json`](./config.public.json) for a list of all trusted users.

# How does ofborg call `nix-build`?

ofborg runs builds with a command similar to the following:

```shell
$ HOME=/homeless-shelter NIX_PATH=ofborg-nixpkgs-pr=$(pwd) nix-build ./default.nix \
    -A hello \
    --no-out-link \
    --keep-going \
    --option restrict-eval true \ 
    --option build-timeout 1800 \ 
    --argstr system thesystem \
    --show-trace
```

# How does ofborg call `nix-instantiate`?

ofborg runs NixOS evals with a command similar to the following:

```shell
$ HOME=/homeless-shelter NIX_PATH=ofborg-nixpkgs-pr=$(pwd) nix-instantiate ./nixos/release.nix \
    -A manual \
    --option restrict-eval true \
    --option build-timeout 1800 \
    --argstr system thesystem \
    --show-trace
```

ofborg runs Nixpkgs evals with a command similar to the following:

```shell
$ HOME=/homeless-shelter NIX_PATH=ofborg-nixpkgs-pr=$(pwd) nix-instantiate ./pkgs/top-level/release.nix \
    -A manual \
    --option restrict-eval true \
    --option build-timeout 1800 \
    --argstr system thesystem \
    --show-trace
```

# Running meta checks locally

To run the meta checks, you will need the
[`outpaths.nix`](./ofborg/src/outpaths.nix) file. You can acquire this file and
run the checks themselves like so:

```shell
$ curl -o outpaths.nix https://raw.githubusercontent.com/NixOS/ofborg/released/ofborg/src/outpaths.nix
$ GC_INITIAL_HEAP_SIZE=4g nix-env -f ./outpaths.nix -qaP --no-name --out-path --arg checkMeta true > out-paths
```

# Hacking

```shell
$ git clone https://github.com/NixOS/ofborg/
$ cd ofborg
$ nix-shell ./shell.nix
$ cd ofborg # enter the subdirectory with Rust code
# make your changes
$ cargo build
$ cargo check
$ cargo test
```

To test whether or not Continuous Integration will pass with your changes, you
can run the following commands from the root of your checkout:

```shell
$ nix-shell --pure --run checkPhase # checks rustfmt, clippy & runs the test suite
$ nix-build -A ofborg.rs # build ofborg
```

Currently there is no easy way to set up a test instance of ofborg. If `cargo
check` and `cargo test` both succeed, feel free to Pull Request your changes.
Make sure to format your code with `cargo fmt` and check for additional warnings
with `cargo clippy`.

To disable warnings as errors, run your command with an empty `RUSTFLAGS`. For
example:

```shell
$ RUSTFLAGS= cargo clippy
```

This will override the default of `-D warnings` set in
[`shell.nix`](./shell.nix), which tells Rust to error if it detects any
warnings.

# Running a builder

If you want to run a builder of your own, check out the [wiki page on operating
a builder](https://github.com/NixOS/ofborg/wiki/Operating-a-Builder/).
