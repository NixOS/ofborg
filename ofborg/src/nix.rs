use std::path::Path;
use std::ffi::OsString;
use std::process::{Command,Stdio};
use tempfile::tempfile;
use std::fs::File;
use std::io::Seek;
use std::io::SeekFrom;

pub struct Nix {
    system: String,
    remote: String
}

pub fn new(system: String, remote: String) -> Nix {
    return Nix{
        system: system,
        remote: remote,
    }
}

impl Nix {
    pub fn safely_build_attrs(&self, nixpkgs: &Path, attrs: Vec<String>) -> Result<File,File> {
        let mut nixpath = OsString::new();
        nixpath.push("nixpkgs=");
        nixpath.push(nixpkgs.as_os_str());

        let mut attrargs: Vec<String> = Vec::with_capacity(attrs.len() * 2);
        for attr in attrs {
            attrargs.push(String::from("-A"));
            attrargs.push(attr);
        }

        let stdout = tempfile().expect("Fetching a stdout tempfile");
        let stderr = stdout.try_clone().expect("Cloning stdout for stderr");
        let mut reader = stderr.try_clone().expect("Cloning stderr to the reader");

        let status = Command::new("nix-build")
            .env_clear()
            .current_dir(nixpkgs)
            .stdout(Stdio::from(stdout))
            .stderr(Stdio::from(stderr))
            .env("NIX_PATH", nixpath)
            .env("NIX_REMOTE", &self.remote)
            .arg("--no-out-link")
            .args(&["--option", "restrict-eval", "true"])
            .args(&["--argstr", "system", &self.system])
            .arg("--keep-going")
            .args(attrargs)
            .status()
            .expect("Running nix-build");

        reader.seek(SeekFrom::Start(0)).expect("Seeking to Start(0)");

        if status.success() {
            return Ok(reader)
        } else {
            return Err(reader)
        }
    }
}
