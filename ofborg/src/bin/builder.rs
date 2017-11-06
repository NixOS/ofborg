extern crate ofborg;
extern crate amqp;

use std::path::Path;
use amqp::Basic;
use amqp::protocol;
use amqp::Session;
use amqp::Table;
use std::process;

use ofborg::checkout;

fn main() {
    println!("Hello, world!");


    let cloner = checkout::cached_cloner(Path::new("/home/grahamc/.nix-test-rs"));
    let project = cloner.project("NixOS/nixpkgs".to_string(),
                                 "https://github.com/nixos/nixpkgs.git".to_string()
    );
    let co = project.clone_for("builder".to_string(),
                               "1234".to_string()).unwrap();

    let refpath = co.checkout_ref("origin/master".as_ref());
    co.fetch_pr(31228).unwrap();
    co.merge_commit("7214d0f6f7a6467205761f87973140727154e1b3".as_ref()).unwrap();

    match refpath {
        Ok(path) => {
            println!("Got path: {:?}", path);
        }
        Err(wat) => {
            println!("Failed to do a checkout of ref : {:?}", wat);
        }
    }



    if false {
        let mut session = Session::open_url("amqps://grahamc:cCbKQmwnRcd8kvPW9cjmMSkp@events.nix.gsc.io//").unwrap();
        let mut channel = session.open_channel(1).unwrap();

        //queue: &str, passive: bool, durable: bool, exclusive: bool, auto_delete: bool, nowait: bool, arguments: Table
        if let Err(problem) = channel.queue_declare("my_queue_name", false, true, false, false, false, Table::new()) {
            println!("Failed to declare a queue: {:?}", problem);
            process::exit(1);
        }

        if let Err(result) = channel.basic_publish("", "my_queue_name", true, false,
                                                   protocol::basic::BasicProperties{ content_type: Some("text".to_string()), ..Default::default()}, (b"Hello from rust!").to_vec()) {
            println!("Failed to publish: {:?}", result);
            process::exit(1);
        }
    }
}
