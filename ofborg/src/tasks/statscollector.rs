use crate::stats;
use crate::worker;

use tracing::error;

pub struct StatCollectorWorker<E> {
    events: E,
    collector: stats::MetricCollector,
}

impl<E: stats::SysEvents + 'static> StatCollectorWorker<E> {
    pub fn new(events: E, collector: stats::MetricCollector) -> StatCollectorWorker<E> {
        StatCollectorWorker { events, collector }
    }
}

impl<E: stats::SysEvents + 'static> worker::SimpleWorker for StatCollectorWorker<E> {
    type J = stats::EventMessage;

    fn msg_to_job(&mut self, _: &str, _: &Option<String>, body: &[u8]) -> Result<Self::J, String> {
        match serde_json::from_slice(body) {
            Ok(e) => Ok(e),
            Err(_) => {
                let mut modified_body: Vec<u8> = vec![b"\""[0]];
                modified_body.append(&mut body.to_vec());
                modified_body.push(b"\""[0]);

                match serde_json::from_slice(&modified_body) {
                    Ok(event) => {
                        self.events.notify(stats::Event::StatCollectorLegacyEvent(
                            stats::event_metric_name(&event),
                        ));
                        Ok(stats::EventMessage {
                            sender: "".to_owned(),
                            events: vec![event],
                        })
                    }
                    Err(err) => {
                        self.events.notify(stats::Event::StatCollectorBogusEvent);
                        error!(
                            "Failed to decode message: {:?}, Err: {err:?}",
                            String::from_utf8(body.to_vec())
                        );
                        Err("Failed to decode message".to_owned())
                    }
                }
            }
        }
    }

    fn consumer(&mut self, job: &stats::EventMessage) -> worker::Actions {
        let sender = job.sender.clone();
        for event in job.events.iter() {
            self.collector.record(sender.clone(), event.clone());
        }

        vec![worker::Action::Ack]
    }
}
