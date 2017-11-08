use std::path::Path;
use std::ffi::OsString;
use std::process::{Command,Stdio};
use tempfile::tempfile;
use std::fs::File;

pub fn safely_build_attrs(nixpkgs: &Path, attrs: Vec<String>) -> Result<File,File> {
    let mut nixpath = OsString::new();
    nixpath.push("nixpkgs=");
    nixpath.push(nixpkgs.as_os_str());

    let stdout = tempfile().unwrap();
    let stderr = stdout.try_clone().unwrap();
    let reader = stderr.try_clone().unwrap();

    let mut cmd = Command::new("nix-build")
        .env_clear()
        .current_dir(nixpkgs)
        .stdout(Stdio::from(stdout))
        .stderr(Stdio::from(stderr))
        .env("NIX_PATH", nixpath);

    for attr in attrs {
        cmd.arg("-A");
        cmd.arg(attr);
    }

    let stat = cmd
        .status()
        .unwrap();


    return Ok(reader);
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
