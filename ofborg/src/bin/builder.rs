extern crate ofborg;
extern crate amqp;

use std::env;
use amqp::{Consumer, Channel};
use amqp::protocol::basic::{Deliver,BasicProperties};

use std::path::Path;
use amqp::Basic;
use amqp::protocol;
use amqp::Session;
use amqp::Table;
use std::process;
use std::io::Error;

use ofborg::config;
use ofborg::checkout;
use ofborg::worker;
use ofborg::message::buildjob;
use ofborg::nix;

fn main() {
    let cfg = config::load(env::args().nth(1).unwrap().as_ref());

    println!("Hello, world!");


    let mut session = Session::open_url(&cfg.rabbitmq.as_uri()).unwrap();
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
        "build-inputs-samples",
        "lmao1",
        false,
        false,
        false,
        false,
        Table::new()
    );

    channel.start_consuming();

    channel.close(200, "Bye").unwrap();
    session.close(200, "Good Bye");

}

struct BuildWorker {
    cloner: checkout::CachedCloner,
}

impl worker::SimpleWorker for BuildWorker {
    type J = buildjob::BuildJob;
    type A = buildjob::Actions;

    fn msg_to_job(&self, method: &Deliver, headers: &BasicProperties,
                  body: &Vec<u8>) -> Result<Self::J, String> {
        println!("lmao I got a job?");
        return match buildjob::from(body) {
            Ok(e) => { return Ok(e) }
            Err(e) => {
                println!("{:?}", String::from_utf8(body.clone()));
                panic!("{:?}", e);
            }
        }
    }

    fn job_to_actions(&self, channel: &mut amqp::Channel, job: &buildjob::BuildJob) -> buildjob::Actions {
        return buildjob::Actions{};
    }


    fn consumer(&self, job: buildjob::BuildJob, resp: buildjob::Actions) -> Result<(), Error> {
        let project = self.cloner.project(job.repo.full_name, job.repo.clone_url);
        let co = project.clone_for("builder".to_string(),
                                   job.pr.number.to_string())?;

        let target_branch = match job.pr.target_branch {
            Some(x) => { x }
            None => { String::from("origin/master") }
        };

        let refpath = co.checkout_ref(target_branch.as_ref()).unwrap();
        co.fetch_pr(job.pr.number).unwrap();
        co.merge_commit(job.pr.head_sha.as_ref()).unwrap();

        println!("Got path: {:?}", refpath);

        let cmd = nix::safely_build_attrs_cmd(refpath, job.attrs);

        return Ok(())
    }
}
