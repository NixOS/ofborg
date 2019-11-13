extern crate amqp;
extern crate env_logger;

use crate::nix;
use std::fs::File;
use std::path::Path;

pub struct EvalChecker {
    name: String,
    op: nix::Operation,
    args: Vec<String>,
    nix: nix::Nix,
}

impl EvalChecker {
    pub fn new(name: &str, op: nix::Operation, args: Vec<String>, nix: nix::Nix) -> EvalChecker {
        EvalChecker {
            name: name.to_owned(),
            op,
            args,
            nix,
        }
    }

    pub fn name(&self) -> String {
        format!("grahamcofborg-eval-{}", self.name)
    }

    pub fn execute(&self, path: &Path) -> Result<File, File> {
        self.nix.safely(&self.op, path, self.args.clone(), false)
    }

    pub fn cli_cmd(&self) -> String {
        let mut cli = vec![self.op.to_string()];
        cli.append(&mut self.args.clone());
        cli.join(" ")
    }
}
