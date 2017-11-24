use std::path::Path;
use std::ffi::OsString;
use std::process::{Command,Stdio};
use tempfile::tempfile;
use std::fs::File;
use std::io::Seek;
use std::io::SeekFrom;

#[derive(Clone, Debug, PartialEq)]
pub struct Nix {
    system: String,
    remote: String,
    build_timeout: u16
}

impl Nix {
    pub fn new(system: String, remote: String, build_timeout: u16) -> Nix {
        return Nix{
            system: system,
            remote: remote,
            build_timeout: build_timeout,
        }
    }

    pub fn with_system(&self, system: String) -> Nix {
        return Nix{
            system: system,
            remote: self.remote.clone(),
            build_timeout: self.build_timeout,
        };
    }

    pub fn safely_build_attrs(&self, nixpkgs: &Path, file: &str, attrs: Vec<String>) -> Result<File,File> {
        let mut attrargs: Vec<String> = Vec::with_capacity(3 + (attrs.len() * 2));
        attrargs.push(file.to_owned());
        attrargs.push(String::from("--no-out-link"));
        attrargs.push(String::from("--keep-going"));
        for attr in attrs {
            attrargs.push(String::from("-A"));
            attrargs.push(attr);
        }

        return self.safely("nix-build", nixpkgs, attrargs);
    }

    pub fn safely(&self, cmd: &str, nixpkgs: &Path, args: Vec<String>) -> Result<File,File> {
        let mut nixpath = OsString::new();
        nixpath.push("nixpkgs=");
        nixpath.push(nixpkgs.as_os_str());

        let stdout = tempfile().expect("Fetching a stdout tempfile");
        let stderr = stdout.try_clone().expect("Cloning stdout for stderr");
        let mut reader = stderr.try_clone().expect("Cloning stderr to the reader");

        let status = Command::new(cmd)
            .env_clear()
            .current_dir(nixpkgs)
            .stdout(Stdio::from(stdout))
            .stderr(Stdio::from(stderr))
            .env("HOME", "/homeless-shelter")
            .env("NIX_PATH", nixpath)
            .env("NIX_REMOTE", &self.remote)
            .args(&["--option", "restrict-eval", "true"])
            .args(&["--option", "build-timeout", &format!("{}", self.build_timeout)])
            .args(&["--argstr", "system", &self.system])
            .args(args)
            .status()
            .expect(format!("Running {:?}", cmd).as_ref());

        reader.seek(SeekFrom::Start(0)).expect("Seeking to Start(0)");

        if status.success() {
            return Ok(reader)
        } else {
            return Err(reader)
        }
    }
}
