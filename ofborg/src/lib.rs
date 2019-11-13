#![recursion_limit = "512"]
// Replacing .map(|arch| arch.to_string())
//      with .map(systems::System::to_string)
//
// seems much less clear and I just don't like it :)
#![allow(clippy::redundant_closure)]

#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate serde_json;

#[macro_use]
extern crate log;

#[macro_use]
extern crate nom;

extern crate amqp;
extern crate chrono;
extern crate either;
extern crate env_logger;
extern crate fs2;
extern crate hubcaps;
extern crate hyper;
extern crate hyper_native_tls;
extern crate lru_cache;
extern crate md5;
extern crate separator;
extern crate tempfile;
extern crate uuid;

use std::env;

pub mod acl;
pub mod asynccmd;
pub mod checkout;
pub mod clone;
pub mod commentparser;
pub mod commitstatus;
pub mod config;
pub mod easyamqp;
pub mod evalchecker;
pub mod files;
pub mod ghevent;
pub mod locks;
pub mod maintainers;
pub mod message;
pub mod nix;
pub mod nixenv;
pub mod nixstats;
pub mod notifyworker;
pub mod outpathdiff;
pub mod stats;
pub mod systems;
pub mod tagger;
pub mod tasks;
pub mod test_scratch;
pub mod worker;
pub mod writetoline;

pub mod ofborg {
    pub use crate::acl;
    pub use crate::asynccmd;
    pub use crate::checkout;
    pub use crate::clone;
    pub use crate::commentparser;
    pub use crate::commitstatus;
    pub use crate::config;
    pub use crate::easyamqp;
    pub use crate::evalchecker;
    pub use crate::files;
    pub use crate::ghevent;
    pub use crate::locks;
    pub use crate::message;
    pub use crate::nix;
    pub use crate::notifyworker;
    pub use crate::outpathdiff;
    pub use crate::stats;
    pub use crate::systems;
    pub use crate::tagger;
    pub use crate::tasks;
    pub use crate::test_scratch;
    pub use crate::worker;
    pub use crate::writetoline;

    pub const VERSION: &str = env!("CARGO_PKG_VERSION");

    pub fn partition_result<A, B>(results: Vec<Result<A, B>>) -> (Vec<A>, Vec<B>) {
        let mut ok = Vec::new();
        let mut err = Vec::new();
        for result in results.into_iter() {
            match result {
                Ok(x) => {
                    ok.push(x);
                }
                Err(x) => {
                    err.push(x);
                }
            }
        }

        (ok, err)
    }
}

pub fn setup_log() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
        env_logger::init().unwrap();
        info!("Defaulting RUST_LOG environment variable to info");
    } else {
        env_logger::init().unwrap();
    }
}
