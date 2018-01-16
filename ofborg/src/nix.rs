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
        let cmd = self.safely_build_attrs_cmd(nixpkgs, file, attrs);

        return self.run(cmd, true);
    }

    pub fn safely_build_attrs_cmd(&self, nixpkgs: &Path, file: &str, attrs: Vec<String>) -> Command {
        let mut attrargs: Vec<String> = Vec::with_capacity(3 + (attrs.len() * 2));
        attrargs.push(file.to_owned());
        attrargs.push(String::from("--no-out-link"));
        attrargs.push(String::from("--keep-going"));
        for attr in attrs {
            attrargs.push(String::from("-A"));
            attrargs.push(attr);
        }

        return self.safe_command("nix-build", nixpkgs, attrargs);
    }

    pub fn safely(&self, cmd: &str, nixpkgs: &Path, args: Vec<String>, keep_stdout: bool) -> Result<File,File> {
        return self.run(self.safe_command(cmd, nixpkgs, args), keep_stdout);
    }

    pub fn run(&self, mut cmd: Command, keep_stdout: bool) -> Result<File,File> {
        let stderr = tempfile().expect("Fetching a stderr tempfile");
        let mut reader = stderr.try_clone().expect("Cloning stderr to the reader");

        let stdout: Stdio;

        if keep_stdout {
            let stdout_fd = stderr.try_clone().expect("Cloning stderr for stdout");
            stdout = Stdio::from(stdout_fd);
        } else {
            stdout = Stdio::null();
        }

        let status = cmd
            .stdout(Stdio::from(stdout))
            .stderr(Stdio::from(stderr))
            .status()
            .expect(format!("Running a program ...").as_ref());

        reader.seek(SeekFrom::Start(0)).expect("Seeking to Start(0)");

        if status.success() {
            return Ok(reader)
        } else {
            return Err(reader)
        }
    }

    pub fn safe_command(&self, cmd: &str, nixpkgs: &Path, args: Vec<String>) -> Command {
        let mut nixpath = OsString::new();
        nixpath.push("nixpkgs=");
        nixpath.push(nixpkgs.as_os_str());

        let mut command = Command::new(cmd);
        command.env_clear();
        command.current_dir(nixpkgs);
        command.env("HOME", "/homeless-shelter");
        command.env("NIX_PATH", nixpath);
        command.env("NIX_REMOTE", &self.remote);
        command.args(&["--show-trace"]);
        command.args(&["--option", "restrict-eval", "true"]);
        command.args(&["--option", "build-timeout", &format!("{}", self.build_timeout)]);
        command.args(&["--argstr", "system", &self.system]);
        command.args(args);

        return command;
    }

}

#[cfg(test)]
mod tests {
    fn nix() -> Nix {
        Nix::new("x86_64-linux".to_owned(), "daemon".to_owned(), 1800)
    }

    fn build_path() -> PathBuf {
        let mut cwd = env::current_dir().unwrap();
        cwd.push(Path::new("./test-srcs/build"));
        return cwd;
    }

    fn passing_eval_path() -> PathBuf {
        let mut cwd = env::current_dir().unwrap();
        cwd.push(Path::new("./test-srcs/eval"));
        return cwd;
    }

    #[derive(Debug)]
    enum Expect {
        Pass,
        Fail,
    }

    fn assert_run(res: Result<File,File>, expected: Expect, require: Vec<&str>) {
        let expectation_held: bool = match expected {
            Expect::Pass => res.is_ok(),
            Expect::Fail => res.is_err(),
        };

        let file: File = match res {
            Ok(file) => file,
            Err(file) => file,
        };

        let lines: Vec<String> = BufReader::new(file)
            .lines()
            .into_iter()
            .filter(|line| line.is_ok())
            .map(|line| line.unwrap())
            .collect();

        let buildlog = lines
            .into_iter()
            .map(|line| format!("   | {}", line))
            .collect::<Vec<String>>()
            .join("\n");

        let total_requirements = require.len();
        let mut missed_requirements: usize = 0;
        let requirements_held: Vec<Result<String, String>> =
            require.into_iter()
            .map(|line| line.to_owned())
            .map(|line|
                 if buildlog.contains(&line) {
                     Ok(line)
                 } else {
                     missed_requirements += 1;
                     Err(line)
                 }
            )
            .collect();

        let mut prefixes: Vec<String> = vec![
            "".to_owned(),
            "".to_owned(),
        ];

        if !expectation_held {
            prefixes.push(format!(
                "The run was expected to {:?}, but did not.",
                expected
            ));
            prefixes.push("".to_owned());
        } else {
            prefixes.push(format!(
                "The run was expected to {:?}, and did.",
                expected
            ));
            prefixes.push("".to_owned());
        }

        let mut suffixes = vec![
            "".to_owned(),
            format!("{} out of {} required lines matched.",
                    (total_requirements - missed_requirements),
                    total_requirements
            ),
            "".to_owned(),
        ];

        for expected_line in requirements_held {
            suffixes.push(format!(" - {:?}", expected_line));
        }
        suffixes.push("".to_owned());

        let output_blocks: Vec<Vec<String>> = vec![
            prefixes,
            vec![buildlog, "".to_owned()],
            suffixes,
        ];

        let output_blocks_strings: Vec<String> =
            output_blocks
            .into_iter()
            .map(|lines| lines.join("\n"))
            .collect();

        let output: String = output_blocks_strings
            .join("\n");

        if expectation_held && missed_requirements == 0 {
        } else {
            panic!(output);
        }
    }

    use super::*;
    use std::io::BufReader;
    use std::io::BufRead;
    use std::path::PathBuf;
    use std::env;

    #[test]
    fn safely_build_attrs_success() {
        let nix = nix();

        let ret: Result<File,File>  = nix.safely_build_attrs(
            build_path().as_path(),
            "default.nix",
            vec![String::from("success")]
        );

        assert_run(ret, Expect::Pass, vec![
            "-success.drv",
            "building path(s)",
            "hi",
            "-success"
        ]);
    }

    #[test]
    fn safely_build_attrs_failure() {
        let nix = nix();

        let ret: Result<File,File>  = nix.safely_build_attrs(
            build_path().as_path(),
            "default.nix",
            vec![String::from("failed")]
        );

        assert_run(ret, Expect::Fail, vec![
            "-failed.drv",
            "building path(s)",
            "hi",
            "failed to produce output path"
        ]);
    }

    #[test]
    fn strict_sandboxing() {
        let ret: Result<File,File>  = nix().safely_build_attrs(
            build_path().as_path(),
            "default.nix",
            vec![String::from("sandbox-violation")]
        );

        assert_run(ret, Expect::Fail, vec![
            "error: while evaluating the attribute",
            "access to path",
            "is forbidden in restricted mode"
        ]);
    }


    #[test]
    fn instantiation() {
        let ret: Result<File,File>  = nix().safely(
            "nix-instantiate",
            passing_eval_path().as_path(),
            vec![],
            true
        );

        assert_run(ret, Expect::Pass, vec![
            "the result might be removed by the garbage collector",
            "-failed.drv",
            "-success.drv"
        ]);
    }
}
