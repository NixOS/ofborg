use std::collections::HashMap;
use std::io::{self, BufRead, BufReader, Read};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::mpsc::{self, sync_channel, Receiver, SyncSender};
use std::thread::{self, JoinHandle};

use tracing::{debug, error, info};

// Specifically set to fall under 1/2 of the AMQP library's
// SyncSender limitation.
const OUT_CHANNEL_BUFFER_SIZE: usize = 30;

// The waiter channel should never be over 3 items: process, stderr,
// stdout, and thusly probably could be unbounded just fine, but what
// the heck.
const WAITER_CHANNEL_BUFFER_SIZE: usize = 10;

pub struct AsyncCmd {
    command: Command,
}

pub struct SpawnedAsyncCmd {
    waiter: JoinHandle<Option<Result<ExitStatus, io::Error>>>,
    rx: Receiver<String>,
}

#[derive(Debug, Hash, PartialEq, Eq)]
enum WaitTarget {
    Stderr,
    Stdout,
    Child,
}

#[derive(Debug)]
enum WaitResult<T> {
    Thread(thread::Result<T>),
    Process(Result<ExitStatus, io::Error>),
}

fn reader_tx<R: 'static + Read + Send>(read: R, tx: SyncSender<String>) -> thread::JoinHandle<()> {
    let read = BufReader::new(read);

    thread::spawn(move || {
        for line in read.lines() {
            let to_send: String = match line {
                Ok(line) => line,
                Err(e) => {
                    error!("Error reading data in reader_tx: {:?}", e);
                    "Non-UTF8 data omitted from the log.".to_owned()
                }
            };

            if let Err(e) = tx.send(to_send) {
                error!("Failed to send log line: {:?}", e);
            }
        }
    })
}

fn spawn_join<T: Send + 'static>(
    id: WaitTarget,
    tx: SyncSender<(WaitTarget, WaitResult<T>)>,
    waiting_on: thread::JoinHandle<T>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        if let Err(e) = tx.send((id, WaitResult::Thread(waiting_on.join()))) {
            error!("Failed to send message to the thread waiter: {:?}", e);
        }
    })
}

fn child_wait<T: Send + 'static>(
    id: WaitTarget,
    tx: SyncSender<(WaitTarget, WaitResult<T>)>,
    mut waiting_on: Child,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        if let Err(e) = tx.send((id, WaitResult::Process(waiting_on.wait()))) {
            error!("Failed to send message to the thread waiter: {:?}", e);
        }
    })
}

impl AsyncCmd {
    pub fn new(cmd: Command) -> AsyncCmd {
        AsyncCmd { command: cmd }
    }

    pub fn spawn(mut self) -> SpawnedAsyncCmd {
        let mut child = self
            .command
            .stdin(Stdio::null())
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        let (monitor_tx, monitor_rx) = sync_channel(WAITER_CHANNEL_BUFFER_SIZE);
        let (proc_tx, proc_rx) = sync_channel(OUT_CHANNEL_BUFFER_SIZE);

        let mut waiters: HashMap<WaitTarget, thread::JoinHandle<()>> = HashMap::with_capacity(3);
        waiters.insert(
            WaitTarget::Stderr,
            spawn_join(
                WaitTarget::Stderr,
                monitor_tx.clone(),
                reader_tx(child.stderr.take().unwrap(), proc_tx.clone()),
            ),
        );

        waiters.insert(
            WaitTarget::Stdout,
            spawn_join(
                WaitTarget::Stdout,
                monitor_tx.clone(),
                reader_tx(child.stdout.take().unwrap(), proc_tx),
            ),
        );

        waiters.insert(
            WaitTarget::Child,
            child_wait(WaitTarget::Child, monitor_tx, child),
        );

        let head_waiter = thread::spawn(move || block_on_waiters(monitor_rx, waiters));

        SpawnedAsyncCmd {
            waiter: head_waiter,
            rx: proc_rx,
        }
    }
}

impl SpawnedAsyncCmd {
    pub fn lines(&mut self) -> mpsc::Iter<'_, String> {
        self.rx.iter()
    }

    pub fn wait(self) -> Result<ExitStatus, io::Error> {
        self.waiter
            .join()
            .map_err(|_err| io::Error::new(io::ErrorKind::Other, "Couldn't join thread."))
            .and_then(|opt| {
                opt.ok_or_else(|| {
                    io::Error::new(io::ErrorKind::Other, "Thread didn't return an exit status.")
                })
            })
            .and_then(|res| res)
    }
}

// FIXME: remove with rust/cargo update
#[allow(clippy::cognitive_complexity)]
fn block_on_waiters(
    monitor_rx: mpsc::Receiver<(WaitTarget, WaitResult<()>)>,
    mut waiters: HashMap<WaitTarget, thread::JoinHandle<()>>,
) -> Option<Result<ExitStatus, io::Error>> {
    let mut status = None;

    for (id, interior_result) in monitor_rx.iter() {
        match waiters.remove(&id) {
            Some(handle) => {
                info!("Received notice that {:?} finished", id);
                let waiter_result = handle.join();

                info!("waiter status: {:?}", waiter_result);
                info!("interior status: {:?}", interior_result);

                match interior_result {
                    WaitResult::Thread(t) => {
                        debug!("thread result: {:?}", t);
                    }
                    WaitResult::Process(t) => {
                        status = Some(t);
                    }
                }
            }
            None => {
                error!(
                    "Received notice that {:?} finished, but it isn't being waited on?",
                    id
                );
            }
        }

        if waiters.is_empty() {
            debug!("Closing up the waiter receiver thread, no more waiters.");
            break;
        }
    }

    info!(
        "Out of the child waiter recv, with {:?} remaining waits",
        waiters.len()
    );

    status
}

#[cfg(test)]
mod tests {
    use super::AsyncCmd;
    use std::ffi::{OsStr, OsString};
    use std::os::unix::ffi::OsStrExt;
    use std::process::Command;

    #[test]
    fn basic_echo_test() {
        let mut cmd = Command::new("/bin/sh");
        cmd.arg("-c");
        cmd.arg("echo hi");
        let acmd = AsyncCmd::new(cmd);

        let mut spawned = acmd.spawn();
        let lines: Vec<String> = spawned.lines().collect();
        assert_eq!(lines, vec!["hi"]);
        let exit_status = spawned.wait().unwrap();
        assert!(exit_status.success());
    }

    #[test]
    fn basic_interpolation_test() {
        let mut cmd = Command::new("stdbuf");
        cmd.arg("-o0");
        cmd.arg("-e0");
        cmd.arg("bash");
        cmd.arg("-c");

        // The sleep 0's are to introduce delay between output to help
        // make it more predictably received in the right order
        cmd.arg("echo stdout; sleep 0.1; echo stderr >&2; sleep 0.1; echo stdout2; sleep 0.1; echo stderr2 >&2");
        let acmd = AsyncCmd::new(cmd);

        let mut spawned = acmd.spawn();
        let lines: Vec<String> = spawned.lines().collect();
        assert_eq!(lines, vec!["stdout", "stderr", "stdout2", "stderr2"]);
        let exit_status = spawned.wait().unwrap();
        assert!(exit_status.success());
    }

    #[test]
    fn lots_of_small_ios_test() {
        let mut cmd = Command::new("/bin/sh");
        cmd.arg("-c");
        cmd.arg("for i in `seq 1 100`; do (seq 1 100)& (seq 1 100 >&2)& wait; wait; done");
        let acmd = AsyncCmd::new(cmd);

        let mut spawned = acmd.spawn();
        let lines: Vec<String> = spawned.lines().collect();
        assert_eq!(lines.len(), 20000);
        let thread_result = spawned.wait();
        let exit_status = thread_result.expect("Thread should exit correctly");
        assert!(exit_status.success());
    }

    #[test]
    fn lots_of_io_test() {
        let mut cmd = Command::new("/bin/sh");
        cmd.arg("-c");
        cmd.arg("seq 1 100000; seq 1 100000 >&2");
        let acmd = AsyncCmd::new(cmd);

        let mut spawned = acmd.spawn();
        let lines: Vec<String> = spawned.lines().collect();
        assert_eq!(lines.len(), 200000);
        let thread_result = spawned.wait();
        let exit_status = thread_result.expect("Thread should exit correctly");
        assert!(exit_status.success());
    }

    #[test]
    fn bad_utf8_test() {
        let mut echos = OsString::from("echo hi; echo ");
        echos.push(OsStr::from_bytes(&[0xffu8]));
        echos.push("; echo there;");

        let mut cmd = Command::new("/bin/sh");
        cmd.arg("-c");
        cmd.arg(echos);
        let acmd = AsyncCmd::new(cmd);

        let mut spawned = acmd.spawn();
        let lines: Vec<String> = spawned.lines().collect();
        assert_eq!(
            lines,
            vec!["hi", "Non-UTF8 data omitted from the log.", "there"]
        );
        let exit_status = spawned.wait().unwrap();
        assert!(exit_status.success());
    }
}
