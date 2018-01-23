extern crate amqp;
extern crate env_logger;

use lru_cache::LruCache;
use serde_json;
use std::fs;
use std::fs::{OpenOptions, File};
use std::path::{Component, PathBuf};

use ofborg::writetoline;
use ofborg::message::buildlogmsg::BuildLogMsg;
use ofborg::worker;
use amqp::protocol::basic::{Deliver, BasicProperties};

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub struct LogFrom {
    routing_key: String,
    attempt_id: String,
}

pub struct LogMessageCollector {
    handles: LruCache<LogFrom, File>,
    log_root: PathBuf,
}

#[derive(Debug)]
pub struct LogMessage {
    from: LogFrom,
    message: BuildLogMsg,
}

fn validate_path_segment(segment: &PathBuf) -> Result<(), String> {
    let components = segment.components();

    if components.count() == 0 {
        return Err(String::from("Segment has no components"));
    }

    if segment.components().all(|component| match component {
        Component::Normal(_) => true,
        e => {
            println!("Invalid path component: {:?}", e);
            false
        }
    })
    {
        return Ok(());
    } else {
        return Err(String::from("Path contained invalid components"));
    }
}

impl LogMessageCollector {
    pub fn new(log_root: PathBuf, max_open: usize) -> LogMessageCollector {
        return LogMessageCollector {
            handles: LruCache::new(max_open),
            log_root: log_root,
        };
    }

    pub fn handle_for(&mut self, from: &LogFrom) -> Result<&mut File, String> {
        if self.handles.contains_key(&from) {
            return Ok(self.handles.get_mut(&from).expect(
                "handles just contained the key",
            ));
        } else {
            let logpath = self.path_for(&from)?;
            let handle = self.open_log(logpath)?;
            self.handles.insert(from.clone(), handle);
            if let Some(handle) = self.handles.get_mut(&from) {
                return Ok(handle);
            } else {
                return Err(String::from(
                    "A just-inserted value should already be there",
                ));
            }
        }
    }

    fn path_for(&self, from: &LogFrom) -> Result<PathBuf, String> {
        let mut location = self.log_root.clone();

        let routing_key = PathBuf::from(from.routing_key.clone());
        validate_path_segment(&routing_key)?;
        location.push(routing_key);

        let attempt_id = PathBuf::from(from.attempt_id.clone());
        validate_path_segment(&attempt_id)?;
        location.push(attempt_id);

        if location.starts_with(&self.log_root) {
            return Ok(location);
        } else {
            return Err(format!(
                "Calculating the log location for {:?} resulted in an invalid path {:?}",
                from,
                location
            ));
        }
    }

    fn open_log(&self, path: PathBuf) -> Result<File, String> {
        let dir = path.parent().unwrap();
        fs::create_dir_all(dir).unwrap();

        let attempt = OpenOptions::new()
            .append(true)
            .read(true)
            .write(true)
            .create(true)
            .open(&path);

        match attempt {
            Ok(handle) => Ok(handle),
            Err(e) => Err(format!(
                "Failed to open the log file for {:?}, err: {:?}",
                &path,
                e
            )),
        }
    }
}

impl worker::SimpleWorker for LogMessageCollector {
    type J = LogMessage;

    fn msg_to_job(
        &mut self,
        deliver: &Deliver,
        _: &BasicProperties,
        body: &Vec<u8>,
    ) -> Result<Self::J, String> {

        let decode = serde_json::from_slice(body);
        if let Err(e) = decode {
            return Err(format!("failed to decode job: {:?}", e));
        }

        let message: BuildLogMsg = decode.unwrap();

        Ok(LogMessage {
            from: LogFrom {
                routing_key: deliver.routing_key.clone(),
                attempt_id: message.attempt_id.clone(),
            },
            message: message,
        })
    }

    fn consumer(&mut self, job: &LogMessage) -> worker::Actions {
        let mut handle = self.handle_for(&job.from).unwrap();

        writetoline::write_to_line(
            &mut handle,
            (job.message.line_number - 1) as usize,
            &job.message.output,
        );

        return vec![worker::Action::Ack];
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;
    use std::io::Read;
    use std::path::Path;
    use ofborg::worker::SimpleWorker;

    fn tpath(component: &str) -> PathBuf {
        return Path::new(env!("CARGO_MANIFEST_DIR")).join(component);
    }

    fn scratch_dir(name: &str) -> PathBuf {
        tpath(&format!("./test-message-log-scratch-{}", name))
    }

    fn cleanup_scratch(name: &str) {
        Command::new("rm")
            .arg("-rf")
            .arg(&scratch_dir(name))
            .status()
            .expect("cleanup of scratch_dir should work");
    }

    fn make_worker(name: &str) -> LogMessageCollector {
        cleanup_scratch(name);

        LogMessageCollector::new(scratch_dir(name), 3)
    }

    fn make_from(id: &str) -> LogFrom {
        LogFrom {
            attempt_id: format!("attempt-id-{}", &id),
            routing_key: format!("routing-key-{}", &id),
        }
    }

    #[test]
    fn test_handle_for() {
        let a = make_from("a.foo/123");
        let b = make_from("b.foo/123");
        let c = make_from("c.foo/123");
        let d = make_from("d.foo/123");

        let mut worker = make_worker("handle_for");
        assert!(worker.handle_for(&a).is_ok());
        assert!(worker.handle_for(&b).is_ok());
        assert!(worker.handle_for(&c).is_ok());
        assert!(worker.handle_for(&d).is_ok());
        assert!(worker.handle_for(&a).is_ok());
    }

    #[test]
    fn test_path_for() {
        let worker = make_worker("path_for");

        let path = worker
            .path_for(&LogFrom {
                attempt_id: String::from("my-attempt-id"),
                routing_key: String::from("my-routing-key"),
            })
            .expect("the path should be valid");


        assert!(path.starts_with(scratch_dir("path_for")));
        assert!(path.ends_with("my-routing-key/my-attempt-id"));
    }

    #[test]
    fn test_path_for_malicious() {
        let worker = make_worker("for_malicious");

        let path = worker.path_for(&LogFrom {
            attempt_id: String::from("./../../"),
            routing_key: String::from("./../../foobar"),
        });

        println!("path: {:?}", path);
        assert!(path.is_err());
    }

    #[test]
    fn test_validate_path_segment() {
        assert!(validate_path_segment(&PathBuf::from("foo")).is_ok());
        assert!(validate_path_segment(&PathBuf::from("foo/bar")).is_ok());
        assert!(validate_path_segment(&PathBuf::from("foo.bar/123")).is_ok());
        assert!(validate_path_segment(&PathBuf::from("..")).is_err());
        assert!(validate_path_segment(&PathBuf::from(".")).is_err());
        assert!(validate_path_segment(&PathBuf::from("./././")).is_err());
        assert!(validate_path_segment(&PathBuf::from("")).is_err());
        assert!(validate_path_segment(&PathBuf::from("foo/..")).is_err());
        assert!(validate_path_segment(&PathBuf::from("foo/../bar")).is_err());
        assert!(validate_path_segment(&PathBuf::from("foo/./bar")).is_ok());
        assert!(validate_path_segment(&PathBuf::from("/foo/bar")).is_err());
        assert!(validate_path_segment(&PathBuf::from("/foo")).is_err());
    }


    #[test]
    fn test_open_log() {
        let worker = make_worker("open-log");
        assert!(
            worker
                .open_log(worker.path_for(&make_from("a")).unwrap())
                .is_ok()
        );
        assert!(
            worker
                .open_log(worker.path_for(&make_from("b.foo/123")).unwrap())
                .is_ok()
        );
    }

    #[test]
    pub fn test_logs_collect() {
        let mut job = LogMessage {
            from: make_from("foo"),
            message: BuildLogMsg {
                attempt_id: String::from("my-attempt-id"),
                identity: String::from("my-identity"),
                system: String::from("foobar-x8664"),
                line_number: 1,
                output: String::from("line-1"),
            },
        };

        {
            let mut worker = make_worker("simple-build");
            assert_eq!(vec![worker::Action::Ack], worker.consumer(&job));

            job.message.line_number = 5;
            job.message.output = String::from("line-5");
            assert_eq!(vec![worker::Action::Ack], worker.consumer(&job));

            job.from.attempt_id = String::from("my-other-attempt");
            job.message.attempt_id = String::from("my-other-attempt");
            job.message.line_number = 3;
            job.message.output = String::from("line-3");
            assert_eq!(vec![worker::Action::Ack], worker.consumer(&job));
        }

        let root = scratch_dir("simple-build");

        let mut p = root.clone();
        let mut s = String::new();
        p.push("routing-key-foo/attempt-id-foo");
        File::open(p).unwrap().read_to_string(&mut s).unwrap();
        assert_eq!(&s, "line-1\n\n\n\nline-5\n");


        let mut p = root.clone();
        let mut s = String::new();
        p.push("routing-key-foo/my-other-attempt");
        File::open(p).unwrap().read_to_string(&mut s).unwrap();
        assert_eq!(&s, "\n\nline-3\n");
    }
}
