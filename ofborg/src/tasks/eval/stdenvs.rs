use crate::files::file_to_str;
use crate::nix;
use std::path::PathBuf;

enum StdenvFrom {
    Before,
    After,
}

#[derive(Debug)]
pub enum System {
    X8664Darwin,
    X8664Linux,
}

#[derive(Debug, PartialEq)]
pub struct Stdenvs {
    nix: nix::Nix,
    co: PathBuf,

    linux_stdenv_before: Option<String>,
    linux_stdenv_after: Option<String>,

    darwin_stdenv_before: Option<String>,
    darwin_stdenv_after: Option<String>,
}

impl Stdenvs {
    pub fn new(nix: nix::Nix, co: PathBuf) -> Stdenvs {
        Stdenvs {
            nix,
            co,

            linux_stdenv_before: None,
            linux_stdenv_after: None,

            darwin_stdenv_before: None,
            darwin_stdenv_after: None,
        }
    }

    pub fn identify_before(&mut self) {
        self.identify(System::X8664Linux, StdenvFrom::Before);
        self.identify(System::X8664Darwin, StdenvFrom::Before);
    }

    pub fn identify_after(&mut self) {
        self.identify(System::X8664Linux, StdenvFrom::After);
        self.identify(System::X8664Darwin, StdenvFrom::After);
    }

    pub fn are_same(&self) -> bool {
        self.changed().is_empty()
    }

    pub fn changed(&self) -> Vec<System> {
        let mut changed: Vec<System> = vec![];

        if self.linux_stdenv_before != self.linux_stdenv_after {
            changed.push(System::X8664Linux);
        }

        if self.darwin_stdenv_before != self.darwin_stdenv_after {
            changed.push(System::X8664Darwin);
        }

        changed
    }

    fn identify(&mut self, system: System, from: StdenvFrom) {
        match (system, from) {
            (System::X8664Linux, StdenvFrom::Before) => {
                self.linux_stdenv_before = self.evalstdenv("x86_64-linux");
            }
            (System::X8664Linux, StdenvFrom::After) => {
                self.linux_stdenv_after = self.evalstdenv("x86_64-linux");
            }

            (System::X8664Darwin, StdenvFrom::Before) => {
                self.darwin_stdenv_before = self.evalstdenv("x86_64-darwin");
            }
            (System::X8664Darwin, StdenvFrom::After) => {
                self.darwin_stdenv_after = self.evalstdenv("x86_64-darwin");
            }
        }
    }

    /// This is used to find out what the output path of the stdenv for the
    /// given system.
    fn evalstdenv(&self, system: &str) -> Option<String> {
        let result = self.nix.with_system(system.to_owned()).safely(
            &nix::Operation::QueryPackagesOutputs,
            &self.co,
            vec![
                String::from("-f"),
                String::from("."),
                String::from("-A"),
                String::from("stdenv"),
            ],
            true,
        );

        println!("{:?}", result);

        match result {
            Ok(mut out) => Some(file_to_str(&mut out)),
            Err(mut out) => {
                println!("{:?}", file_to_str(&mut out));
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::env;
    use std::process::Command;

    #[test]
    fn stdenv_checking() {
        let output = Command::new("nix-instantiate")
            .args(&["--eval", "-E", "<nixpkgs>"])
            .output()
            .expect("nix-instantiate required");

        let nixpkgs = String::from_utf8(output.stdout).expect("nixpkgs required");

        let remote = env::var("NIX_REMOTE").unwrap_or("".to_owned());
        let nix = nix::Nix::new(String::from("x86_64-linux"), remote, 1200, None);
        let mut stdenv = Stdenvs::new(nix.clone(), PathBuf::from(nixpkgs.trim_end()));
        stdenv.identify(System::X8664Linux, StdenvFrom::Before);
        stdenv.identify(System::X8664Darwin, StdenvFrom::Before);

        stdenv.identify(System::X8664Linux, StdenvFrom::After);
        stdenv.identify(System::X8664Darwin, StdenvFrom::After);

        assert!(stdenv.are_same());
    }
}
