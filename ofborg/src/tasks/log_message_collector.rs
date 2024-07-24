use crate::message::buildlogmsg::{BuildLogMsg, BuildLogStart};
use crate::message::buildresult::BuildResult;
use crate::worker;
use crate::writetoline::LineWriter;

use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Component, Path, PathBuf};

use lru_cache::LruCache;
use tracing::warn;

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub struct LogFrom {
    routing_key: String,
    attempt_id: String,
}

pub struct LogMessageCollector {
    handles: LruCache<LogFrom, LineWriter>,
    log_root: PathBuf,
}

#[derive(Debug)]
enum MsgType {
    Start(BuildLogStart),
    Msg(BuildLogMsg),
    Finish(Box<BuildResult>),
}

#[derive(Debug)]
pub struct LogMessage {
    from: LogFrom,
    message: MsgType,
}

fn validate_path_segment(segment: &Path) -> Result<(), String> {
    let components = segment.components();

    if components.count() == 0 {
        return Err(String::from("Segment has no components"));
    }

    if segment.components().all(|component| match component {
        Component::Normal(_) => true,
        e => {
            warn!("Invalid path component: {:?}", e);
            false
        }
    }) {
        Ok(())
    } else {
        Err(String::from("Path contained invalid components"))
    }
}

impl LogMessageCollector {
    pub fn new(log_root: PathBuf, max_open: usize) -> LogMessageCollector {
        LogMessageCollector {
            handles: LruCache::new(max_open),
            log_root,
        }
    }

    pub fn write_metadata(&mut self, from: &LogFrom, data: &BuildLogStart) -> Result<(), String> {
        let metapath = self.path_for_metadata(from)?;
        let mut fp = self.open_file(&metapath)?;

        match serde_json::to_string(data) {
            Ok(data) => {
                if let Err(e) = fp.write(data.as_bytes()) {
                    Err(format!("Failed to write metadata: {:?}", e))
                } else {
                    Ok(())
                }
            }
            Err(e) => Err(format!("Failed to stringify metadata: {:?}", e)),
        }
    }

    pub fn write_result(&mut self, from: &LogFrom, data: &BuildResult) -> Result<(), String> {
        let path = self.path_for_result(from)?;
        let mut fp = self.open_file(&path)?;

        match serde_json::to_string(data) {
            Ok(data) => {
                if let Err(e) = fp.write(data.as_bytes()) {
                    Err(format!("Failed to write result: {:?}", e))
                } else {
                    Ok(())
                }
            }
            Err(e) => Err(format!("Failed to stringify result: {:?}", e)),
        }
    }

    pub fn handle_for(&mut self, from: &LogFrom) -> Result<&mut LineWriter, String> {
        if self.handles.contains_key(from) {
            Ok(self
                .handles
                .get_mut(from)
                .expect("handles just contained the key"))
        } else {
            let logpath = self.path_for_log(from)?;
            let fp = self.open_file(&logpath)?;
            let writer = LineWriter::new(fp);
            self.handles.insert(from.clone(), writer);
            if let Some(handle) = self.handles.get_mut(from) {
                Ok(handle)
            } else {
                Err(String::from(
                    "A just-inserted value should already be there",
                ))
            }
        }
    }

    fn path_for_metadata(&self, from: &LogFrom) -> Result<PathBuf, String> {
        let mut path = self.path_for_log(from)?;
        path.set_extension("metadata.json");
        Ok(path)
    }

    fn path_for_result(&self, from: &LogFrom) -> Result<PathBuf, String> {
        let mut path = self.path_for_log(from)?;
        path.set_extension("result.json");
        Ok(path)
    }

    fn path_for_log(&self, from: &LogFrom) -> Result<PathBuf, String> {
        let mut location = self.log_root.clone();

        let routing_key = PathBuf::from(from.routing_key.clone());
        validate_path_segment(&routing_key)?;
        location.push(routing_key);

        let attempt_id = PathBuf::from(from.attempt_id.clone());
        validate_path_segment(&attempt_id)?;
        location.push(attempt_id);

        if location.starts_with(&self.log_root) {
            Ok(location)
        } else {
            Err(format!(
                "Calculating the log location for {:?} resulted in an invalid path {:?}",
                from, location
            ))
        }
    }

    fn open_file(&self, path: &Path) -> Result<File, String> {
        let dir = path.parent().unwrap();
        fs::create_dir_all(dir).unwrap();

        let attempt = OpenOptions::new()
            .append(true)
            .read(true)
            .write(true)
            .create(true)
            .open(path);

        match attempt {
            Ok(handle) => Ok(handle),
            Err(e) => Err(format!(
                "Failed to open the file for {:?}, err: {:?}",
                &path, e
            )),
        }
    }
}

impl worker::SimpleWorker for LogMessageCollector {
    type J = LogMessage;

    fn msg_to_job(
        &mut self,
        routing_key: &str,
        _: &Option<String>,
        body: &[u8],
    ) -> Result<Self::J, String> {
        let message: MsgType;
        let attempt_id: String;

        let decode_msg: Result<BuildLogMsg, _> = serde_json::from_slice(body);
        if let Ok(msg) = decode_msg {
            attempt_id = msg.attempt_id.clone();
            message = MsgType::Msg(msg);
        } else {
            let decode_msg: Result<BuildLogStart, _> = serde_json::from_slice(body);
            if let Ok(msg) = decode_msg {
                attempt_id = msg.attempt_id.clone();
                message = MsgType::Start(msg);
            } else {
                let decode_msg: Result<BuildResult, _> = serde_json::from_slice(body);
                if let Ok(msg) = decode_msg {
                    attempt_id = msg.legacy().attempt_id;
                    message = MsgType::Finish(Box::new(msg));
                } else {
                    return Err(format!("failed to decode job: {:?}", decode_msg));
                }
            }
        }

        Ok(LogMessage {
            from: LogFrom {
                routing_key: routing_key.to_string(),
                attempt_id,
            },
            message,
        })
    }

    fn consumer(&mut self, job: &LogMessage) -> worker::Actions {
        match job.message {
            MsgType::Start(ref start) => {
                self.write_metadata(&job.from, start)
                    .expect("failed to write metadata");

                // Make sure the log content exists by opening its handle.
                // This (hopefully) prevents builds that produce no output (for any reason) from
                // having their logs.nix.ci link complaining about a 404.
                let _ = self.handle_for(&job.from).unwrap();
            }
            MsgType::Msg(ref message) => {
                let handle = self.handle_for(&job.from).unwrap();

                handle.write_to_line((message.line_number - 1) as usize, &message.output);
            }
            MsgType::Finish(ref finish) => {
                self.write_result(&job.from, finish)
                    .expect("failed to write result");
            }
        }

        vec![worker::Action::Ack]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::buildresult::{BuildStatus, V1Tag};
    use crate::message::{Pr, Repo};
    use crate::test_scratch::TestScratch;
    use crate::worker::SimpleWorker;
    use std::io::Read;
    use std::path::PathBuf;

    fn make_worker(path: PathBuf) -> LogMessageCollector {
        LogMessageCollector::new(path, 3)
    }

    fn make_from(id: &str) -> LogFrom {
        LogFrom {
            attempt_id: format!("attempt-id-{}", &id),
            routing_key: format!("routing-key-{}", &id),
        }
    }

    #[test]
    fn test_handle_for() {
        let p = TestScratch::new_dir("log-message-collector-handle_for");

        let a = make_from("a.foo/123");
        let b = make_from("b.foo/123");
        let c = make_from("c.foo/123");
        let d = make_from("d.foo/123");

        let mut worker = make_worker(p.path());
        assert!(worker.handle_for(&a).is_ok());
        assert!(worker.handle_for(&b).is_ok());
        assert!(worker.handle_for(&c).is_ok());
        assert!(worker.handle_for(&d).is_ok());
        assert!(worker.handle_for(&a).is_ok());
    }

    #[test]
    fn test_path_for_metadata() {
        let p = TestScratch::new_dir("log-message-collector-path_for_metadata");
        let worker = make_worker(p.path());

        let path = worker
            .path_for_metadata(&LogFrom {
                attempt_id: String::from("my-attempt-id"),
                routing_key: String::from("my-routing-key"),
            })
            .expect("the path should be valid");

        assert!(path.starts_with(p.path()));
        assert!(path
            .as_os_str()
            .to_string_lossy()
            .ends_with("my-routing-key/my-attempt-id.metadata.json"));
    }

    #[test]
    fn test_path_for_result() {
        let p = TestScratch::new_dir("log-message-collector-path_for_result");
        let worker = make_worker(p.path());

        let path = worker
            .path_for_result(&LogFrom {
                attempt_id: String::from("my-attempt-id"),
                routing_key: String::from("my-routing-key"),
            })
            .expect("the path should be valid");

        assert!(path.starts_with(p.path()));
        assert!(path
            .as_os_str()
            .to_string_lossy()
            .ends_with("my-routing-key/my-attempt-id.result.json"));
    }

    #[test]
    fn test_path_for_log() {
        let p = TestScratch::new_dir("log-message-collector-path_for_log");
        let worker = make_worker(p.path());

        let path = worker
            .path_for_log(&LogFrom {
                attempt_id: String::from("my-attempt-id"),
                routing_key: String::from("my-routing-key"),
            })
            .expect("the path should be valid");

        assert!(path.starts_with(p.path()));
        assert!(path.ends_with("my-routing-key/my-attempt-id"));
    }

    #[test]
    fn test_path_for_log_malicious() {
        let p = TestScratch::new_dir("log-message-collector-for_malicious");
        let worker = make_worker(p.path());

        let path = worker.path_for_log(&LogFrom {
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
    fn test_open_file() {
        let p = TestScratch::new_dir("log-message-collector-open_file");
        let worker = make_worker(p.path());

        assert!(worker
            .open_file(&worker.path_for_log(&make_from("a")).unwrap())
            .is_ok());
        assert!(worker
            .open_file(&worker.path_for_log(&make_from("b.foo/123")).unwrap())
            .is_ok());
    }

    #[test]
    pub fn test_logs_collect() {
        let mut logmsg = BuildLogMsg {
            attempt_id: String::from("my-attempt-id"),
            identity: String::from("my-identity"),
            system: String::from("foobar-x8664"),
            line_number: 1,
            output: String::from("line-1"),
        };
        let mut job = LogMessage {
            from: make_from("foo"),
            message: MsgType::Msg(logmsg.clone()),
        };

        let p = TestScratch::new_dir("log-message-collector-logs_collector");

        {
            let mut worker = make_worker(p.path());
            assert_eq!(
                vec![worker::Action::Ack],
                worker.consumer(&LogMessage {
                    from: make_from("foo"),
                    message: MsgType::Start(BuildLogStart {
                        attempt_id: String::from("my-attempt-id"),
                        identity: String::from("my-identity"),
                        system: String::from("foobar-x8664"),
                        attempted_attrs: Some(vec!["foo".to_owned()]),
                        skipped_attrs: Some(vec!["bar".to_owned()]),
                    })
                })
            );

            assert!(p.path().join("routing-key-foo/attempt-id-foo").exists());
            assert_eq!(vec![worker::Action::Ack], worker.consumer(&job));

            logmsg.line_number = 5;
            logmsg.output = String::from("line-5");
            job.message = MsgType::Msg(logmsg.clone());
            assert_eq!(vec![worker::Action::Ack], worker.consumer(&job));

            job.from.attempt_id = String::from("my-other-attempt");
            logmsg.attempt_id = String::from("my-other-attempt");
            logmsg.line_number = 3;
            logmsg.output = String::from("line-3");
            job.message = MsgType::Msg(logmsg);
            assert_eq!(vec![worker::Action::Ack], worker.consumer(&job));

            assert_eq!(
                vec![worker::Action::Ack],
                worker.consumer(&LogMessage {
                    from: make_from("foo"),
                    message: MsgType::Finish(Box::new(BuildResult::V1 {
                        tag: V1Tag::V1,
                        repo: Repo {
                            clone_url: "https://github.com/nixos/ofborg.git".to_owned(),
                            full_name: "NixOS/ofborg".to_owned(),
                            owner: "NixOS".to_owned(),
                            name: "ofborg".to_owned(),
                        },
                        pr: Pr {
                            number: 42,
                            head_sha: "6dd9f0265d52b946dd13daf996f30b64e4edb446".to_owned(),
                            target_branch: Some("scratch".to_owned()),
                        },
                        system: "x86_64-linux".to_owned(),
                        output: vec![],
                        attempt_id: "attempt-id-foo".to_owned(),
                        request_id: "bogus-request-id".to_owned(),
                        status: BuildStatus::Success,
                        attempted_attrs: Some(vec!["foo".to_owned()]),
                        skipped_attrs: Some(vec!["bar".to_owned()]),
                    }))
                })
            );
        }

        let mut prm = p.path();
        let mut sm = String::new();
        prm.push("routing-key-foo/attempt-id-foo.metadata.json");
        File::open(prm).unwrap().read_to_string(&mut sm).unwrap();
        assert_eq!(&sm, "{\"system\":\"foobar-x8664\",\"identity\":\"my-identity\",\"attempt_id\":\"my-attempt-id\",\"attempted_attrs\":[\"foo\"],\"skipped_attrs\":[\"bar\"]}");

        let mut prf = p.path();
        let mut sf = String::new();
        prf.push("routing-key-foo/attempt-id-foo");
        File::open(prf).unwrap().read_to_string(&mut sf).unwrap();
        assert_eq!(&sf, "line-1\n\n\n\nline-5\n");

        let mut pr = p.path();
        let mut s = String::new();
        pr.push("routing-key-foo/my-other-attempt");
        File::open(pr).unwrap().read_to_string(&mut s).unwrap();
        assert_eq!(&s, "\n\nline-3\n");

        let mut prr = p.path();
        let mut sr = String::new();
        prr.push("routing-key-foo/attempt-id-foo.result.json");
        File::open(prr).unwrap().read_to_string(&mut sr).unwrap();
        assert_eq!(&sr, "{\"tag\":\"V1\",\"repo\":{\"owner\":\"NixOS\",\"name\":\"ofborg\",\"full_name\":\"NixOS/ofborg\",\"clone_url\":\"https://github.com/nixos/ofborg.git\"},\"pr\":{\"target_branch\":\"scratch\",\"number\":42,\"head_sha\":\"6dd9f0265d52b946dd13daf996f30b64e4edb446\"},\"system\":\"x86_64-linux\",\"output\":[],\"attempt_id\":\"attempt-id-foo\",\"request_id\":\"bogus-request-id\",\"status\":\"Success\",\"skipped_attrs\":[\"bar\"],\"attempted_attrs\":[\"foo\"]}");
    }
}
