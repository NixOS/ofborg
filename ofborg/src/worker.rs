
use amqp::{Consumer, Channel};
use amqp::protocol::basic::{Deliver,BasicProperties};
use std::marker::Send;
use std::io::Error;

pub struct Worker<T: SimpleWorker> {
    internal: T
}

pub struct StdRepo {
    pub full_name: String,
    pub clone_url: String,
}

pub struct StdPr {
    pub target_branch: String,
    pub number: i64,
    pub head_sha: String,
}


pub struct BuildJob {
    pub repo: StdRepo,
    pub pr: StdPr,
}


pub struct Actions {
}

pub trait SimpleWorker {
    fn consumer(&self, job: BuildJob, resp: Actions) -> Result<(), Error>;
}

pub fn new<T: SimpleWorker>(worker: T) -> Worker<T> {
    return Worker{
        internal: worker,
    };
}

impl <T: SimpleWorker + Send> Consumer for Worker<T> {
    fn handle_delivery(&mut self,
                       channel: &mut Channel,
                       method: Deliver,
                       headers: BasicProperties,
                       body: Vec<u8>) {
    }
}
