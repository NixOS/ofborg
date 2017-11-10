extern crate ofborg;
extern crate amqp;
extern crate env_logger;

use std::collections::LinkedList;
use std::env;
use amqp::protocol::basic::{Deliver,BasicProperties};

use std::path::Path;
use amqp::Basic;
use amqp::Session;
use amqp::Table;
use std::process;
use std::io::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use ofborg::config;
use ofborg::checkout;
use ofborg::worker;
use ofborg::message::buildjob;
use ofborg::nix;

fn main() {
    let cfg = config::load(env::args().nth(1).unwrap().as_ref());
    env_logger::init().unwrap();
    println!("Hello, world!");


    let mut session = Session::open_url(&cfg.rabbitmq.as_uri()).unwrap();
    let mut channel = session.open_channel(1).unwrap();

    //queue: &str, passive: bool, durable: bool, exclusive: bool, auto_delete: bool, nowait: bool, arguments: Table
    if let Err(problem) = channel.queue_declare("my_queue_name", false, true, false, false, false, Table::new()) {
        println!("Failed to declare a queue: {:?}", problem);
        process::exit(1);
    }

    let cloner = checkout::cached_cloner(Path::new(&cfg.checkout.root));
    let nix = nix::new(cfg.nix.system.clone(), cfg.nix.remote);

    channel.basic_consume(
        worker::new(BuildWorker{
            cloner: cloner,
            nix: nix,
            system: cfg.nix.system.clone()
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
    nix: nix::Nix,
    system: String,
}

impl BuildWorker {
    fn actions(&self) -> buildjob::Actions {
        return buildjob::Actions{
            system: self.system.clone(),
        };
    }
}

impl worker::SimpleWorker for BuildWorker {
    type J = buildjob::BuildJob;

    fn msg_to_job(&self, _: &Deliver, _: &BasicProperties,
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

    fn consumer(&self, job: &buildjob::BuildJob) -> worker::Actions {
        let project = self.cloner.project(job.repo.full_name.clone(), job.repo.clone_url.clone());
        let co = project.clone_for("builder".to_string(),
                                   job.pr.number.to_string()).unwrap();

        let target_branch = match job.pr.target_branch.clone() {
            Some(x) => { x }
            None => { String::from("origin/master") }
        };

        let refpath = co.checkout_ref(target_branch.as_ref()).unwrap();
        co.fetch_pr(job.pr.number).unwrap();
        co.merge_commit(job.pr.head_sha.as_ref()).unwrap();

        println!("Got path: {:?}, building", refpath);


        let success: bool;
        let reader: BufReader<File>;
        match self.nix.safely_build_attrs(refpath.as_ref(), job.attrs.clone()) {
            Ok(r) => {
                success = true;
                reader = BufReader::new(r);
            }
            Err(r) => {
                success = false;
                reader = BufReader::new(r);
            }
        }
        println!("ok built ({:?}), building", success);

        let l10 = reader.lines().fold(LinkedList::new(),

                                      |mut coll, line|
                                      {
                                          match line {
                                              Ok(e) => { coll.push_back(e); }
                                              Err(wtf) => {
                                                  println!("Got err in lines: {:?}", wtf);
                                                  coll.push_back(String::from("<line omitted due to error>"));
                                              }
                                          }

                                          if coll.len() == 11 {
                                              coll.pop_front();
                                          }

                                          return coll
                                      }
        );
        println!("Lines: {:?}", l10);

        let last10lines = l10.into_iter().collect::<Vec<_>>();


        return self.actions().build_finished(
            &job,
            success,
            last10lines.clone()
        );
    }
}
