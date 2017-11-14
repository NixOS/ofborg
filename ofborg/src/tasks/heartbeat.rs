

use std::{thread, time};
use serde_json;
use ofborg::worker;
use ofborg::message::plasticheartbeat;
use amqp::Channel;
use amqp::Table;
use amqp::protocol::basic::{Deliver,BasicProperties};
use std::process;
use amqp::Basic;

struct PlasticHeartbeatWorker {
    queue_name: String
}

impl PlasticHeartbeatWorker {
    fn message(&self) -> worker::QueueMsg {
        return worker::QueueMsg{
            exchange: None,
            routing_key: Some(self.queue_name.clone()),
            mandatory: true,
            immediate: false,
            properties: None,
            content: serde_json::to_string(&plasticheartbeat::PlasticHeartbeat{}).unwrap().into_bytes()
        };
    }

}

impl worker::SimpleWorker for PlasticHeartbeatWorker {
    type J = plasticheartbeat::PlasticHeartbeat;

    fn msg_to_job(&self, _: &Deliver, _: &BasicProperties,
                  body: &Vec<u8>) -> Result<Self::J, String> {
        return match plasticheartbeat::from(body) {
            Ok(e) => { Ok(e) }
            Err(e) => {
                println!("{:?}", String::from_utf8(body.clone()));
                panic!("{:?}", e);
            }
        }
    }

    fn consumer(&self, _job: &plasticheartbeat::PlasticHeartbeat) -> worker::Actions {
        thread::sleep(time::Duration::from_secs(5));

        return vec![
            worker::Action::Publish(self.message()),
            worker::Action::Ack
        ];
    }
}

pub fn start_on_channel(mut hbchan: Channel, consumer_name: String) {
    let queue_name = hbchan.queue_declare(
        "",
        false, // passive
        false, // durable
        true, // exclusive
        true, // auto_delete
        false, //nowait
        Table::new()
    )
        .expect("Failed to declare an anon queue for PlasticHeartbeats!")
        .queue;

    println!("Got personal queue: {:?}", queue_name);

    hbchan.basic_publish(
        "",
        queue_name.as_ref(),
        true, // mandatory
        false, // immediate
        BasicProperties {
            ..Default::default()
        },
        serde_json::to_string(&plasticheartbeat::PlasticHeartbeat{}).unwrap().into_bytes()
    ).unwrap();

    let worker = move ||
    {
        hbchan.basic_consume(
            worker::new(
                PlasticHeartbeatWorker{
                    queue_name: (&queue_name).clone()
                }
            ),
            queue_name,
            String::from(format!("{}-heartbeat", consumer_name)),
            false,
            false,
            false,
            false,
            Table::new()
        ).unwrap();

        hbchan.start_consuming();
        println!("PlasticHeartbeat failed");
        process::exit(1);
    };

    thread::spawn(worker);
}
