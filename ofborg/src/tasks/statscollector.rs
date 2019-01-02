extern crate amqp;
extern crate env_logger;

use serde_json;
use ofborg::worker;
use ofborg::stats;
use amqp::protocol::basic::{Deliver, BasicProperties};

pub struct StatCollectorWorker<E> {
    events: E,
    collector: stats::MetricCollector,
}

impl<E: stats::SysEvents + 'static> StatCollectorWorker<E> {
    pub fn new(events: E, collector: stats::MetricCollector) -> StatCollectorWorker<E> {
        StatCollectorWorker {
            events,
            collector,
        }
    }
}

impl<E: stats::SysEvents + 'static> worker::SimpleWorker for StatCollectorWorker<E> {
    type J = stats::EventMessage;

    fn msg_to_job(
        &mut self,
        _: &Deliver,
        _: &BasicProperties,
        body: &Vec<u8>,
    ) -> Result<Self::J, String> {
        return match serde_json::from_slice(body) {
            Ok(e) => Ok(e),
            Err(_) => {
                let mut modified_body: Vec<u8> = vec!["\"".as_bytes()[0]];
                modified_body.append(&mut body.clone());
                modified_body.push("\"".as_bytes()[0]);

                match serde_json::from_slice(&modified_body) {
                    Ok(e) => {
                        self.events.notify(stats::Event::StatCollectorLegacyEvent(stats::event_metric_name(&e)));
                        Ok(stats::EventMessage {
                            sender: "".to_owned(),
                            events: vec![e],
                        })
                    },
                    Err(e) => {
                        self.events.notify(stats::Event::StatCollectorBogusEvent);
                        error!(
                            "Failed to decode message: {:?}, Err: {:?}",
                            String::from_utf8(body.clone()),
                            e
                        );
                        Err("Failed to decode message".to_owned())
                    }
                }
            }
        };
    }

    fn consumer(&mut self, job: &stats::EventMessage) -> worker::Actions {

        let sender = job.sender.clone();
        for event in job.events.iter() {
            self.collector.record(sender.clone(), event.clone());
        }

        return vec![worker::Action::Ack];
    }
}
