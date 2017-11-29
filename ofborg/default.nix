## To update dependencies use:
# $ carnix -o crates.nix --src ./. Cargo.lock --standalone
(import ./crates.nix).ofborg_0_1_0
