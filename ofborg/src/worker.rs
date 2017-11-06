
use amqp::{Consumer, Channel};
use amqp::protocol::basic::{Deliver,BasicProperties};
use std::marker::Send;
use std::io::Error;

pub struct Worker<T: SimpleWorker> {
    internal: T
}

pub struct Actions {
}

pub trait SimpleWorker {
    type J;
    fn consumer(&self, job: Self::J, resp: Actions) -> Result<(), Error>;

    fn msg_to_job(&self, method: &Deliver, headers: &BasicProperties,
                  body: &Vec<u8>) -> Result<Self::J, String>;
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

        match self.internal.msg_to_job(&method, &headers, &body) {
            Ok(job) => {
                let actions = Actions{};
                match self.internal.consumer(job, actions) {
                    Ok(_) => { /* :) */ }
                    Err(_) => {
                        panic!("failed to run job!");
                    }
                }
            }
            Err(e) => {
                panic!(e);
            }
        }
    }
}
