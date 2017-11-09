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

        let stdout = tempfile().unwrap();
        let stderr = stdout.try_clone().unwrap();
        let mut reader = stderr.try_clone().unwrap();

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
            .unwrap();

        reader.seek(SeekFrom::Start(0)).unwrap();

        if status.success() {
            return Ok(reader)
        } else {
            return Err(reader)
        }
    }
}

/*
        $attrs = array_intersperse(array_values((array)$body->attrs), '-A');
        var_dump($attrs);

        $fillers = implode(" ", array_fill(0, count($attrs), '%s'));

        $cmd = 'NIX_PATH=nixpkgs=%s nix-build --no-out-link --argstr system %s --option restrict-eval true --keep-going . ' . $fillers;
        $args = $attrs;
        array_unshift($args, NIX_SYSTEM);
        array_unshift($args, $pname);


*/
