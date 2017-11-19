extern crate amqp;
extern crate env_logger;

use std::fs::File;
use std::path::Path;
use ofborg::nix;

pub struct EvalChecker {
    name: String,
    cmd: String,
    args: Vec<String>,
    nix: nix::Nix,

}

impl EvalChecker {
    pub fn new(name: &str, cmd: &str, args: Vec<String>, nix: nix::Nix) -> EvalChecker {
        EvalChecker{
            name: name.to_owned(),
            cmd: cmd.to_owned(),
            args: args,
            nix: nix,
        }
    }

    pub fn name(&self) -> String {
        format!("grahamcofborg-eval-{}", self.name)
    }

    pub fn execute(&self, path: &Path) -> Result<File, File> {
        self.nix.safely(&self.cmd, path, self.args.clone())
    }

    pub fn cli_cmd(&self) -> String {
        let mut cli = vec![self.cmd.clone()];
        cli.append(&mut self.args.clone());
        return cli.join(" ");
    }
}
