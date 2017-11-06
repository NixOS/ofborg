extern crate ofborg;
extern crate amqp;

use std::path::Path;
use amqp::Basic;
use amqp::protocol;
use amqp::Session;
use amqp::Table;
use std::process;
use std::io::Error;

use ofborg::checkout;
use ofborg::worker;
use ofborg::worker::{Actions,StdPr,StdRepo};


pub struct BuildJob {
    pub repo: StdRepo,
    pub pr: StdPr,
}


fn main() {
    println!("Hello, world!");


    let mut session = Session::open_url("amqps://grahamc:cCbKQmwnRcd8kvPW9cjmMSkp@events.nix.gsc.io//").unwrap();
    let mut channel = session.open_channel(1).unwrap();

    //queue: &str, passive: bool, durable: bool, exclusive: bool, auto_delete: bool, nowait: bool, arguments: Table
    if let Err(problem) = channel.queue_declare("my_queue_name", false, true, false, false, false, Table::new()) {
        println!("Failed to declare a queue: {:?}", problem);
        process::exit(1);
    }

    let cloner = checkout::cached_cloner(Path::new("/home/grahamc/.nix-test-rs"));

    channel.basic_consume(
        worker::new(BuildWorker{
            cloner: cloner
        }),
        "my_queue_name",
        "lmao1",
        false,
        false,
        false,
        false,
        Table::new()
    );

    if let Err(result) = channel.basic_publish("", "my_queue_name", true, false,
                                               protocol::basic::BasicProperties{ content_type: Some("text".to_string()), ..Default::default()}, (b"Hello from rust!").to_vec()) {
        println!("Failed to publish: {:?}", result);
        process::exit(1);
    }
}

struct BuildWorker {
    cloner: checkout::CachedCloner,
}

impl worker::SimpleWorker for BuildWorker {
    type J = BuildJob;

    fn consumer(&self, job: BuildJob, resp: Actions) -> Result<(), Error> {
        let project = self.cloner.project(job.repo.full_name, job.repo.clone_url);
        let co = project.clone_for("builder".to_string(),
                                   job.pr.number.to_string())?;

        let refpath = co.checkout_ref(job.pr.target_branch.as_ref());
        co.fetch_pr(job.pr.number).unwrap();
        co.merge_commit(job.pr.head_sha.as_ref()).unwrap();

        match refpath {
            Ok(path) => {
                println!("Got path: {:?}", path);
            }
            Err(wat) => {
                println!("Failed to do a checkout of ref : {:?}", wat);
            }
        }

        return Ok(())
    }
}
