#[macro_use]
extern crate log;

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use ofborg::config;
use ofborg::nix;

fn main() {
    ofborg::setup_log();

    log::info!("Loading config...");
    let cfg = config::load(env::args().nth(1).unwrap().as_ref());
    let nix = cfg.nix();

    log::info!("Running build...");
    match nix.safely_build_attrs(
        Path::new("./"),
        nix::File::DefaultNixpkgs,
        vec![String::from("hello")],
    ) {
        Ok(mut out) => {
            print!("{}", file_to_str(&mut out));
        }
        Err(mut out) => print!("{}", file_to_str(&mut out)),
    }
}

fn file_to_str(f: &mut File) -> String {
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).expect("Reading eval output");
    String::from(String::from_utf8_lossy(&buffer))
}
