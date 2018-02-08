extern crate prometheus;
extern crate amqp;
extern crate env_logger;

use serde_json;
use std::str::FromStr;
use ofborg::worker;
use ofborg::stats;
use amqp::protocol::basic::{Deliver, BasicProperties};
use std::collections::HashMap;
use std::mem;
use std::thread;
use std::time::Duration;
use std::sync::Arc;
use std::sync::Mutex;

pub struct StatCollectorWorker<E> {
    events: E,
    counter_collectors: HashMap<String, prometheus::CounterVec>,
}

impl<E: stats::SysEvents + 'static> StatCollectorWorker<E> {
    pub fn new(events: E) -> StatCollectorWorker<E> {
        let mut worker = StatCollectorWorker {
            events: events,
            counter_collectors: HashMap::new(),
        };

        let initial_events: Vec<stats::Event> = vec![
            stats::Event::StatCollectorLegacyEvent,
            stats::Event::StatCollectorBogusEvent,
            stats::Event::JobReceived,
            stats::Event::JobDecodeSuccess,
            stats::Event::JobDecodeFailure,
            stats::Event::IssueAlreadyClosed,
            stats::Event::IssueFetchFailed,
        ];
        for initial_event in initial_events {
            match initial_event {
                //                WARNING
                //   BEFORE YOU ADD A NEW VARIANT HERE, ADD IT
                //   TO THE LIST ABOVE!
                //
                //   EACH VARIANT MUST BE INITIALIZED PRIOR
                //   TO REPORTING STATS
                stats::Event::StatCollectorLegacyEvent => {
                    worker.register_counter(
                        &initial_event,
                        prometheus::Opts {
                            namespace: "ofborg".to_owned(),
                            subsystem: "stats_collector".to_owned(),
                            name: "legacy_event".to_owned(),
                            help: "Number of received legacy events".to_owned(),
                            const_labels: HashMap::new(),
                            variable_labels: vec!["instance".to_owned()],
                        }
                    );
                },
                stats::Event::StatCollectorBogusEvent => {
                    worker.register_counter(
                        &initial_event,
                        prometheus::Opts {
                            namespace: "ofborg".to_owned(),
                            subsystem: "stats_collector".to_owned(),
                            name: "bogus_event".to_owned(),
                            help: "Number of received unparseable events".to_owned(),
                            const_labels: HashMap::new(),
                            variable_labels: vec!["instance".to_owned()],
                        }
                    );
                },
                stats::Event::JobReceived => {
                    worker.register_counter(
                        &initial_event,
                        prometheus::Opts {
                            namespace: "ofborg".to_owned(),
                            subsystem: "generic_worker".to_owned(),
                            name: "job_received".to_owned(),
                            help: "Number of received worker jobs".to_owned(),
                            const_labels: HashMap::new(),
                            variable_labels: vec!["instance".to_owned()],
                        }
                    );
                },
                stats::Event::JobDecodeSuccess => {
                    worker.register_counter(
                        &initial_event,
                        prometheus::Opts {
                            namespace: "ofborg".to_owned(),
                            subsystem: "generic_worker".to_owned(),
                            name: "job_decode_successful".to_owned(),
                            help: "Number of successfully decoded jobs".to_owned(),
                            const_labels: HashMap::new(),
                            variable_labels: vec!["instance".to_owned()],
                        }
                    );
                },
                stats::Event::JobDecodeFailure => {
                    worker.register_counter(
                        &initial_event,
                        prometheus::Opts {
                            namespace: "ofborg".to_owned(),
                            subsystem: "generic_worker".to_owned(),
                            name: "job_decode_failure".to_owned(),
                            help: "Number of jobs which failed to parse".to_owned(),
                            const_labels: HashMap::new(),
                            variable_labels: vec!["instance".to_owned()],
                        }
                    );
                },
                stats::Event::IssueAlreadyClosed => {
                    worker.register_counter(
                        &initial_event,
                        prometheus::Opts {
                            namespace: "ofborg".to_owned(),
                            subsystem: "github".to_owned(),
                            name: "issue_closed".to_owned(),
                            help: "Number of jobs for issues which are already closed".to_owned(),
                            const_labels: HashMap::new(),
                            variable_labels: vec!["instance".to_owned()],
                        }

                    );
                },
                stats::Event::IssueFetchFailed => {
                    worker.register_counter(
                        &initial_event,
                        prometheus::Opts {
                            namespace: "ofborg".to_owned(),
                            subsystem: "github".to_owned(),
                            name: "issue_fetch_fail".to_owned(),
                            help: "Number of failed fetches for GitHub issues".to_owned(),
                            const_labels: HashMap::new(),
                            variable_labels: vec!["instance".to_owned()],
                        }
                    );
                },
            };
        }

        return worker;
    }

    pub fn counter(&self, event: &stats::Event) -> prometheus::CounterVec {
        let disc = format!("{:?}", mem::discriminant(event));
        self.counter_collectors.get(&disc).unwrap().clone()
    }

    pub fn register_counter(
        &mut self,
        event: &stats::Event,
        opts: prometheus::Opts,
    ) {
        let disc = format!("{:?}", mem::discriminant(event));
        let orig_labels = opts.variable_labels.clone();
        let labels: Vec<&str> = orig_labels
            .iter()
            .map(|v| v.as_ref())
            .collect();

        let counter = register_counter_vec!(
            opts, labels.as_ref()
        ).unwrap();
        counter.with_label_values(&[""]).inc_by(0.0);

        self.counter_collectors.insert(
            disc,
            counter
        );
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
                        self.events.notify(stats::Event::StatCollectorLegacyEvent);
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
            match *event {
                stats::Event::StatCollectorLegacyEvent => {
                    self.counter(&event).with_label_values(&[sender.as_ref()]).inc();
                },
                stats::Event::StatCollectorBogusEvent => {
                    self.counter(&event).with_label_values(&[sender.as_ref()]).inc();
                },
                stats::Event::JobReceived => {
                    self.counter(&event).with_label_values(&[sender.as_ref()]).inc();
                },
                stats::Event::JobDecodeSuccess => {
                    self.counter(&event).with_label_values(&[sender.as_ref()]).inc();
                },
                stats::Event::JobDecodeFailure => {
                    self.counter(&event).with_label_values(&[sender.as_ref()]).inc();
                },
                stats::Event::IssueAlreadyClosed => {
                    self.counter(&event).with_label_values(&[sender.as_ref()]).inc();
                },
                stats::Event::IssueFetchFailed => {
                    self.counter(&event).with_label_values(&[sender.as_ref()]).inc();
                },
            }
        }

        return vec![worker::Action::Ack];
    }
}
