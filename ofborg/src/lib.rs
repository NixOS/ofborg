#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate log;

extern crate hubcaps;
extern crate hyper;
extern crate hyper_native_tls;

extern crate tempfile;
extern crate amqp;
extern crate fs2;
extern crate md5;

pub mod acl;
pub mod checkout;
pub mod locks;
pub mod clone;
pub mod worker;
pub mod config;
pub mod message;
pub mod tasks;
pub mod evalchecker;
pub mod nix;
pub mod ghevent;
pub mod commentparser;
pub mod commitstatus;
pub mod outpathdiff;
pub mod tagger;
pub mod cmdlog;


pub mod ofborg {
    pub use cmdlog;
    pub use config;
    pub use checkout;
    pub use locks;
    pub use clone;
    pub use worker;
    pub use message;
    pub use tasks;
    pub use evalchecker;
    pub use commitstatus;
    pub use ghevent;
    pub use nix;
    pub use acl;
    pub use commentparser;
    pub use outpathdiff;
    pub use tagger;
}
