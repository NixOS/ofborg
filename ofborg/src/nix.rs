use std::env;
use std::fmt;
use std::fs::File;
use std::io::Seek;
use std::io::SeekFrom;
use std::path::Path;
use std::process::{Command, Stdio};
use tempfile::tempfile;

#[derive(Clone, Debug)]
pub enum Operation {
    Build,
    Instantiate,
    Unknown { program: String },
}

impl Operation {
    pub fn new(program: &str) -> Operation {
        Operation::Unknown { program: program.to_owned() }
    }

    fn command(&self) -> Command {
        match *self {
            Operation::Build => Command::new("nix-build"),
            Operation::Instantiate => Command::new("nix-instantiate"),
            Operation::Unknown { ref program } => Command::new(program),
        }
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Operation::Build => write!(f, "{}", "nix-build"),
            Operation::Instantiate => write!(f, "{}", "nix-instantiate"),
            Operation::Unknown { ref program } => write!(f, "{}", program),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Nix {
    system: String,
    remote: String,
    build_timeout: u16,
    limit_supported_systems: bool,
    initial_heap_size: Option<String>,
}

impl Nix {
    pub fn new(system: String, remote: String, build_timeout: u16, initial_heap_size: Option<String>) -> Nix {
        return Nix {
            system: system,
            remote: remote,
            build_timeout: build_timeout,
            initial_heap_size: initial_heap_size,
            limit_supported_systems: true,
        };
    }

    pub fn with_system(&self, system: String) -> Nix {
        let mut n = self.clone();
        n.system = system;
        return n;
    }

    pub fn with_limited_supported_systems(&self) -> Nix {
        let mut n = self.clone();
        n.limit_supported_systems = true;
        return n;
    }

    pub fn without_limited_supported_systems(&self) -> Nix {
        let mut n = self.clone();
        n.limit_supported_systems = false;
        return n;
    }

    pub fn safely_build_attrs(
        &self,
        nixpkgs: &Path,
        file: &str,
        attrs: Vec<String>,
    ) -> Result<File, File> {
        let cmd = self.safely_build_attrs_cmd(nixpkgs, file, attrs);

        return self.run(cmd, true);
    }

    pub fn safely_build_attrs_cmd(
        &self,
        nixpkgs: &Path,
        file: &str,
        attrs: Vec<String>,
    ) -> Command {
        let mut attrargs: Vec<String> = Vec::with_capacity(3 + (attrs.len() * 2));
        attrargs.push(file.to_owned());
        attrargs.push(String::from("--no-out-link"));
        attrargs.push(String::from("--keep-going"));
        for attr in attrs {
            attrargs.push(String::from("-A"));
            attrargs.push(attr);
        }

        return self.safe_command(Operation::Build, nixpkgs, attrargs);
    }

    pub fn safely(
        &self,
        op: Operation,
        nixpkgs: &Path,
        args: Vec<String>,
        keep_stdout: bool,
    ) -> Result<File, File> {
        return self.run(self.safe_command(op, nixpkgs, args), keep_stdout);
    }

    pub fn run(&self, mut cmd: Command, keep_stdout: bool) -> Result<File, File> {
        let stderr = tempfile().expect("Fetching a stderr tempfile");
        let mut reader = stderr.try_clone().expect("Cloning stderr to the reader");

        let stdout: Stdio;

        if keep_stdout {
            let stdout_fd = stderr.try_clone().expect("Cloning stderr for stdout");
            stdout = Stdio::from(stdout_fd);
        } else {
            stdout = Stdio::null();
        }

        let status = cmd.stdout(Stdio::from(stdout))
            .stderr(Stdio::from(stderr))
            .status()
            .expect(format!("Running a program ...").as_ref());

        reader.seek(SeekFrom::Start(0)).expect(
            "Seeking to Start(0)",
        );

        if status.success() {
            return Ok(reader);
        } else {
            return Err(reader);
        }
    }

    pub fn safe_command(&self, op: Operation, nixpkgs: &Path, args: Vec<String>) -> Command {
        let nixpath = format!("nixpkgs={}", nixpkgs.display());

        let mut command = op.command();
        command.env_clear();
        command.current_dir(nixpkgs);
        command.env("HOME", "/homeless-shelter");
        command.env("NIX_PATH", nixpath);
        command.env("NIX_REMOTE", &self.remote);

        if let Some(ref initial_heap_size) = self.initial_heap_size {
            command.env("GC_INITIAL_HEAP_SIZE", &initial_heap_size);
        }

        let path = env::var("PATH").unwrap();
        command.env("PATH", path);

        command.args(&["--show-trace"]);
        command.args(&["--option", "restrict-eval", "true"]);
        command.args(
            &[
                "--option",
                "build-timeout",
                &format!("{}", self.build_timeout),
            ],
        );
        command.args(&["--argstr", "system", &self.system]);

        if self.limit_supported_systems {
            command.args(
                &[
                    "--arg",
                    "supportedSystems",
                    &format!("[\"{}\"]", &self.system),
                ],
            );
        }

        command.args(args);

        return command;
    }
}

#[cfg(test)]
mod tests {
    fn nix() -> Nix {
        Nix::new("x86_64-linux".to_owned(), "daemon".to_owned(), 1800, None)
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

    fn assert_run(res: Result<File, File>, expected: Expect, require: Vec<&str>) {
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
        let requirements_held: Vec<Result<String, String>> = require
            .into_iter()
            .map(|line| line.to_owned())
            .map(|line| if buildlog.contains(&line) {
                Ok(line)
            } else {
                missed_requirements += 1;
                Err(line)
            })
            .collect();

        let mut prefixes: Vec<String> = vec!["".to_owned(), "".to_owned()];

        if !expectation_held {
            prefixes.push(format!(
                "The run was expected to {:?}, but did not.",
                expected
            ));
            prefixes.push("".to_owned());
        } else {
            prefixes.push(format!("The run was expected to {:?}, and did.", expected));
            prefixes.push("".to_owned());
        }

        let mut suffixes = vec![
            "".to_owned(),
            format!(
                "{} out of {} required lines matched.",
                (total_requirements - missed_requirements),
                total_requirements
            ),
            "".to_owned(),
        ];

        for expected_line in requirements_held {
            suffixes.push(format!(" - {:?}", expected_line));
        }
        suffixes.push("".to_owned());

        let output_blocks: Vec<Vec<String>> =
            vec![prefixes, vec![buildlog, "".to_owned()], suffixes];

        let output_blocks_strings: Vec<String> = output_blocks
            .into_iter()
            .map(|lines| lines.join("\n"))
            .collect();

        let output: String = output_blocks_strings.join("\n");

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
    fn test_build_operation() {
        let nix = nix();
        let op = Operation::Build;

        assert_eq!(op.to_string(), "nix-build");

        let ret: Result<File, File> =
            nix.run(
                nix.safe_command(op, build_path().as_path(), vec![String::from("--version")]),
                true,
            );

        assert_run(
            ret,
            Expect::Pass,
            vec!["nix-build (Nix)"],
        );
    }

    #[test]
    fn test_instantiate_operation() {
        let nix = nix();
        let op = Operation::Instantiate;

        assert_eq!(op.to_string(), "nix-instantiate");

        let ret: Result<File, File> =
            nix.run(
                nix.safe_command(op, build_path().as_path(), vec![String::from("--version")]),
                true,
            );

        assert_run(
            ret,
            Expect::Pass,
            vec!["nix-instantiate (Nix)"],
        );
    }

    #[test]
    fn safe_command_environment() {
        let nix = nix();

        let ret: Result<File, File> =
            nix.run(
                nix.safe_command(Operation::new("./environment.sh"), build_path().as_path(), vec![]),
                true,
            );

        assert_run(
            ret,
            Expect::Pass,
            vec![
                "HOME=/homeless-shelter",
                "NIX_PATH=nixpkgs=",
                "NIX_REMOTE=",
                "PATH=",
            ],
        );
    }

    #[test]
    fn safe_command_custom_gc() {
        let nix = Nix::new("x86_64-linux".to_owned(), "daemon".to_owned(), 1800, Some("4g".to_owned()));

        let ret: Result<File, File> =
            nix.run(
                nix.safe_command(Operation::new("./environment.sh"), build_path().as_path(), vec![]),
                true,
            );

        assert_run(
            ret,
            Expect::Pass,
            vec![
                "HOME=/homeless-shelter",
                "NIX_PATH=nixpkgs=",
                "NIX_REMOTE=",
                "PATH=",
                "GC_INITIAL_HEAP_SIZE=4g",
            ],
        );
    }

    #[test]
    fn safe_command_options() {
        let nix = nix();

        let ret: Result<File, File> = nix.run(
            nix.safe_command(Operation::new("echo"), build_path().as_path(), vec![]),
            true,
        );

        assert_run(
            ret,
            Expect::Pass,
            vec!["--option restrict-eval true", "--option build-timeout 1800"],
        );
    }

    #[test]
    fn safely_build_attrs_success() {
        let nix = nix();

        let ret: Result<File, File> = nix.safely_build_attrs(
            build_path().as_path(),
            "default.nix",
            vec![String::from("success")],
        );

        assert_run(
            ret,
            Expect::Pass,
            vec!["-success.drv", "building ", "hi", "-success"],
        );
    }

    #[test]
    fn safely_build_attrs_failure() {
        let nix = nix();

        let ret: Result<File, File> = nix.safely_build_attrs(
            build_path().as_path(),
            "default.nix",
            vec![String::from("failed")],
        );

        assert_run(
            ret,
            Expect::Fail,
            vec![
                "-failed.drv",
                "building ",
                "hi",
                "failed to produce output path",
            ],
        );
    }

    #[test]
    fn strict_sandboxing() {
        let ret: Result<File, File> = nix().safely_build_attrs(
            build_path().as_path(),
            "default.nix",
            vec![String::from("sandbox-violation")],
        );

        assert_run(
            ret,
            Expect::Fail,
            vec![
                "error: while evaluating the attribute",
                "access to path",
                "is forbidden in restricted mode",
            ],
        );
    }


    #[test]
    fn instantiation() {
        let ret: Result<File, File> = nix().safely(
            Operation::Instantiate,
            passing_eval_path().as_path(),
            vec![],
            true,
        );

        assert_run(
            ret,
            Expect::Pass,
            vec![
                "the result might be removed by the garbage collector",
                "-failed.drv",
                "-success.drv",
            ],
        );
    }
}
