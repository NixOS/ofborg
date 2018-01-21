use std::thread;

use std::process::Stdio;
use std::process::ExitStatus;
use std::sync::mpsc::channel;
use std::process::Command;
use std::io::Read;
use std::sync::mpsc::{Sender, Receiver};
use std::io::BufReader;
use std::io::BufRead;
use std::process::Child;
use std::thread::JoinHandle;

pub struct AsyncCmd {
    command: Command,
}

pub struct SpawnedAsyncCmd {
    stdout_handler: JoinHandle<()>,
    stderr_handler: JoinHandle<()>,
    child: Child,
    rx: Receiver<String>,
}


fn reader_tx<R: 'static + Read + Send>(read: R, tx: Sender<String>) -> thread::JoinHandle<()> {
    let read = BufReader::new(read);

    thread::spawn(move || {
        for line in read.lines() {
            if let Ok(line) = line {
                // println!("sending: {:?}", line);
                tx.send(line).expect("Failed to send log line");
            } else {
                println!("Got in reader tx's else: {:?}", line);
            }
        }
    })
}

impl AsyncCmd {
    pub fn new(cmd: Command) -> AsyncCmd {
        AsyncCmd { command: cmd }
    }

    pub fn spawn(mut self) -> SpawnedAsyncCmd {
        let mut child = self.command
            .stdin(Stdio::null())
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        let (tx, rx) = channel();

        let stderr_handler = reader_tx(child.stderr.take().unwrap(), tx.clone());
        let stdout_handler = reader_tx(child.stdout.take().unwrap(), tx.clone());

        SpawnedAsyncCmd {
            stdout_handler: stdout_handler,
            stderr_handler: stderr_handler,
            child: child,
            rx: rx,
        }
    }
}

impl SpawnedAsyncCmd {
    pub fn lines<'a>(&'a mut self) -> &'a Receiver<String> {
        &self.rx
    }
    pub fn wait(mut self) -> ExitStatus {
        let status = self.child.wait();
        self.stdout_handler.join().unwrap();
        self.stderr_handler.join().unwrap();

        return status.unwrap();
    }
}
