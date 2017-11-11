extern crate amqp;
extern crate env_logger;

use ofborg::ghevent;
use ofborg::acl;
use serde_json;

use ofborg::worker;
use amqp::protocol::basic::{Deliver,BasicProperties};


pub struct BuildFilterWorker {
    acl: acl::ACL,
}

impl BuildFilterWorker {
    pub fn new(acl: acl::ACL) -> BuildFilterWorker {
        return BuildFilterWorker{
            acl: acl,
        };
    }
}

impl worker::SimpleWorker for BuildFilterWorker {
    type J = ghevent::IssueComment;

    fn msg_to_job(&self, _: &Deliver, _: &BasicProperties,
                  body: &Vec<u8>) -> Result<Self::J, String> {
        return match serde_json::from_slice(body) {
            Ok(e) => { Ok(e) }
            Err(e) => {
                println!("Failed to deserialize IsssueComment: {:?}", String::from_utf8(body.clone()));
                panic!("{:?}", e);
            }
        }
    }

    fn consumer(&self, job: &ghevent::IssueComment) -> worker::Actions {
        if !self.acl.can_build(&job.comment.user.login, &job.repository.full_name) {
            println!("ACL prohibits {} from building for {}",
                     job.comment.user.login,
                     job.repository.full_name);
            return vec![
                worker::Action::Ack
            ];
        }

        println!("Got job: {:?}", job);
        return vec![
            worker::Action::NackRequeue
        ];
    }
}
