extern crate ofborg;
extern crate amqp;
extern crate env_logger;

#[macro_use]
extern crate log;

use std::env;

use std::path::Path;
use std::fs::File;
use std::io::Read;
use ofborg::config;


fn main() {
    let cfg = config::load(env::args().nth(1).unwrap().as_ref());

    if let Err(_) = env::var("RUST_LOG") {
        env::set_var("RUST_LOG", "info");
        env_logger::init().unwrap();
        info!("Defaulting RUST_LOG environment variable to info");
    } else {
        env_logger::init().unwrap();
    }

    let nix = cfg.nix();

    match nix.safely_build_attrs(&Path::new("./"), "./default.nix", vec![String::from("hello"),]) {
        Ok(mut out) => { print!("{}", file_to_str(&mut out)); }
        Err(mut out) => { print!("{}", file_to_str(&mut out)) }
    }
}

fn file_to_str(f: &mut File) -> String {
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).expect("Reading eval output");
    return String::from(String::from_utf8_lossy(&buffer));
}
