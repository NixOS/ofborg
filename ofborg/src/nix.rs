use crate::asynccmd::{AsyncCmd, SpawnedAsyncCmd};
use crate::ofborg::partition_result;
use std::collections::HashMap;
use std::env;
use std::ffi::OsStr;
use std::fmt;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Seek;
use std::io::SeekFrom;
use std::path::Path;
use std::process::{Command, Stdio};
use tempfile::tempfile;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum File {
    DefaultNixpkgs,
    ReleaseNixOS,
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            File::DefaultNixpkgs => write!(f, "./default.nix"),
            File::ReleaseNixOS => write!(f, "./nixos/release.nix"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Operation {
    Evaluate,
    Instantiate,
    Build,
    QueryPackagesJSON,
    QueryPackagesOutputs,
    NoOp { operation: Box<Operation> },
    Unknown { program: String },
}

impl Operation {
    fn command(&self) -> Command {
        match *self {
            Operation::Evaluate => Command::new("nix-instantiate"),
            Operation::Instantiate => Command::new("nix-instantiate"),
            Operation::Build => Command::new("nix-build"),
            Operation::QueryPackagesJSON => Command::new("nix-env"),
            Operation::QueryPackagesOutputs => Command::new("nix-env"),
            Operation::NoOp { .. } => Command::new("echo"),
            Operation::Unknown { ref program } => Command::new(program),
        }
    }

    fn args(&self, command: &mut Command) {
        match *self {
            Operation::Build => {
                command.args(&["--no-out-link", "--keep-going"]);
            }
            Operation::QueryPackagesJSON => {
                command.args(&["--query", "--available", "--json"]);
            }
            Operation::QueryPackagesOutputs => {
                command.args(&[
                    "--query",
                    "--available",
                    "--no-name",
                    "--attr-path",
                    "--out-path",
                ]);
            }
            Operation::NoOp { ref operation } => {
                operation.args(command);
            }
            Operation::Evaluate => {
                command.args(&["--eval", "--strict", "--json"]);
            }
            _ => (),
        };
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Operation::Build => write!(f, "nix-build"),
            Operation::Instantiate => write!(f, "nix-instantiate"),
            Operation::QueryPackagesJSON => write!(f, "nix-env -qa --json"),
            Operation::QueryPackagesOutputs => write!(f, "nix-env -qaP --no-name --out-path"),
            Operation::NoOp { ref operation } => operation.fmt(f),
            Operation::Unknown { ref program } => write!(f, "{}", program),
            Operation::Evaluate => write!(f, "nix-instantiate --strict --json ..."),
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
    pub fn new(
        system: String,
        remote: String,
        build_timeout: u16,
        initial_heap_size: Option<String>,
    ) -> Nix {
        Nix {
            system,
            remote,
            build_timeout,
            initial_heap_size,
            limit_supported_systems: true,
        }
    }

    pub fn with_system(&self, system: String) -> Nix {
        let mut n = self.clone();
        n.system = system;
        n
    }

    pub fn with_limited_supported_systems(&self) -> Nix {
        let mut n = self.clone();
        n.limit_supported_systems = true;
        n
    }

    pub fn without_limited_supported_systems(&self) -> Nix {
        let mut n = self.clone();
        n.limit_supported_systems = false;
        n
    }

    pub fn safely_partition_instantiable_attrs(
        &self,
        nixpkgs: &Path,
        file: File,
        attrs: Vec<String>,
    ) -> (Vec<String>, Vec<(String, Vec<String>)>) {
        let attr_instantiations: Vec<Result<String, (String, Vec<String>)>> = attrs
            .into_iter()
            .map(
                |attr| match self.safely_instantiate_attrs(nixpkgs, file, vec![attr.clone()]) {
                    Ok(_) => Ok(attr.clone()),
                    Err(f) => Err((attr.clone(), lines_from_file(f))),
                },
            )
            .collect();

        partition_result(attr_instantiations)
    }

    pub fn safely_instantiate_attrs(
        &self,
        nixpkgs: &Path,
        file: File,
        attrs: Vec<String>,
    ) -> Result<fs::File, fs::File> {
        let mut command = self.safe_command::<&OsStr>(&Operation::Instantiate, nixpkgs, &[], &[]);
        self.set_attrs_command(&mut command, file, attrs);
        self.run(command, true)
    }

    pub fn safely_evaluate_expr_cmd(
        &self,
        nixpkgs: &Path,
        expr: &str,
        argstrs: HashMap<&str, &str>,
        extra_paths: &[&Path],
    ) -> Command {
        let mut attrargs: Vec<String> = Vec::with_capacity(2 + (argstrs.len() * 3));
        attrargs.push("--expr".to_owned());
        attrargs.push(expr.to_owned());
        for (argname, argstr) in argstrs {
            attrargs.push(String::from("--argstr"));
            attrargs.push(argname.to_owned());
            attrargs.push(argstr.to_owned());
        }

        self.safe_command(&Operation::Evaluate, nixpkgs, &attrargs, &extra_paths)
    }

    pub fn safely_build_attrs(
        &self,
        nixpkgs: &Path,
        file: File,
        attrs: Vec<String>,
    ) -> Result<fs::File, fs::File> {
        let mut command = self.safe_command::<&OsStr>(&Operation::Build, nixpkgs, &[], &[]);
        self.set_attrs_command(&mut command, file, attrs);
        self.run(command, true)
    }

    pub fn safely_build_attrs_async(
        &self,
        nixpkgs: &Path,
        file: File,
        attrs: Vec<String>,
    ) -> SpawnedAsyncCmd {
        let mut command = self.safe_command::<&OsStr>(&Operation::Build, nixpkgs, &[], &[]);
        self.set_attrs_command(&mut command, file, attrs);
        AsyncCmd::new(command).spawn()
    }

    fn set_attrs_command(&self, command: &mut Command, file: File, attrs: Vec<String>) {
        let mut args: Vec<String> = Vec::with_capacity(3 + (attrs.len() * 2));
        args.push(format!("{}", file));
        for attr in attrs {
            args.push(String::from("-A"));
            args.push(attr);
        }
        if let File::ReleaseNixOS = file {
            args.push(String::from("--arg"));
            args.push(String::from("nixpkgs"));
            args.push(String::from(
                "{ outPath=./.; revCount=999999; shortRev=\"ofborg\"; }",
            ));
        }
        command.args(args);
    }

    pub fn safely(
        &self,
        op: &Operation,
        nixpkgs: &Path,
        args: Vec<String>,
        keep_stdout: bool,
    ) -> Result<fs::File, fs::File> {
        self.run(self.safe_command(&op, nixpkgs, &args, &[]), keep_stdout)
    }

    pub fn run(&self, mut cmd: Command, keep_stdout: bool) -> Result<fs::File, fs::File> {
        let stderr = tempfile().expect("Fetching a stderr tempfile");
        let mut reader = stderr.try_clone().expect("Cloning stderr to the reader");

        let stdout: Stdio = if keep_stdout {
            Stdio::from(stderr.try_clone().expect("Cloning stderr for stdout"))
        } else {
            Stdio::null()
        };

        let status = cmd
            .stdout(stdout)
            .stderr(Stdio::from(stderr))
            .status()
            .expect("Running a program ...");

        reader
            .seek(SeekFrom::Start(0))
            .expect("Seeking to Start(0)");

        if status.success() {
            Ok(reader)
        } else {
            Err(reader)
        }
    }

    pub fn run_stderr_stdout(&self, mut cmd: Command) -> (bool, fs::File, fs::File) {
        let stdout_file = tempfile().expect("Fetching a stdout tempfile");
        let mut stdout_reader = stdout_file
            .try_clone()
            .expect("Cloning stdout to the reader");

        let stderr_file = tempfile().expect("Fetching a stderr tempfile");
        let mut stderr_reader = stderr_file
            .try_clone()
            .expect("Cloning stderr to the reader");

        let status = cmd
            .stdout(Stdio::from(stdout_file))
            .stderr(Stdio::from(stderr_file))
            .status()
            .expect("Running a program ...");

        stdout_reader
            .seek(SeekFrom::Start(0))
            .expect("Seeking dout to Start(0)");
        stderr_reader
            .seek(SeekFrom::Start(0))
            .expect("Seeking stderr to Start(0)");

        (status.success(), stdout_reader, stderr_reader)
    }

    pub fn safe_command<S>(
        &self,
        op: &Operation,
        nixpkgs: &Path,
        args: &[S],
        safe_paths: &[&Path],
    ) -> Command
    where
        S: AsRef<OsStr>,
    {
        let nixpkgspath = format!("nixpkgs={}", nixpkgs.display());
        let mut nixpath: Vec<String> = safe_paths
            .iter()
            .map(|path| format!("{}", path.display()))
            .collect();
        nixpath.push(nixpkgspath);

        let mut command = op.command();
        op.args(&mut command);

        command.env_clear();
        command.current_dir(nixpkgs);
        command.env("HOME", "/homeless-shelter");
        command.env("NIX_PATH", nixpath.join(":"));
        command.env("NIX_REMOTE", &self.remote);

        if let Some(ref initial_heap_size) = self.initial_heap_size {
            command.env("GC_INITIAL_HEAP_SIZE", &initial_heap_size);
        }

        let path = env::var("PATH").unwrap();
        command.env("PATH", path);

        command.args(&["--show-trace"]);
        command.args(&["--option", "restrict-eval", "true"]);
        command.args(&[
            "--option",
            "build-timeout",
            &format!("{}", self.build_timeout),
        ]);
        command.args(&["--argstr", "system", &self.system]);

        if self.limit_supported_systems {
            command.args(&[
                "--arg",
                "supportedSystems",
                &format!("[\"{}\"]", &self.system),
            ]);
        }

        command.args(args);
        command
    }
}

fn lines_from_file(file: fs::File) -> Vec<String> {
    BufReader::new(file)
        .lines()
        .filter(|line| line.is_ok())
        .map(|line| line.unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    fn nix() -> Nix {
        let remote = env::var("NIX_REMOTE").unwrap_or("".to_owned());
        Nix::new("x86_64-linux".to_owned(), remote, 1800, None)
    }

    fn noop(operation: Operation) -> Operation {
        Operation::NoOp {
            operation: Box::new(operation),
        }
    }

    fn env_noop() -> Operation {
        Operation::Unknown {
            program: "./environment.sh".to_owned(),
        }
    }

    fn build_path() -> PathBuf {
        let mut cwd = env::current_dir().unwrap();
        cwd.push(Path::new("./test-srcs/build"));
        cwd
    }

    fn passing_eval_path() -> PathBuf {
        let mut cwd = env::current_dir().unwrap();
        cwd.push(Path::new("./test-srcs/eval"));
        cwd
    }

    fn individual_eval_path() -> PathBuf {
        let mut cwd = env::current_dir().unwrap();
        cwd.push(Path::new("./test-srcs/eval-mixed-failure"));
        cwd
    }

    fn strip_ansi(string: &str) -> String {
        string
            .replace("‘", "'")
            .replace("’", "'")
            .replace("\u{1b}[31;1m", "") // red
            .replace("\u{1b}[0m", "") // reset
    }

    #[derive(Debug)]
    enum Expect {
        Pass,
        Fail,
    }

    fn assert_run(res: Result<fs::File, fs::File>, expected: Expect, require: Vec<&str>) {
        let expectation_held: bool = match expected {
            Expect::Pass => res.is_ok(),
            Expect::Fail => res.is_err(),
        };

        let file: fs::File = match res {
            Ok(file) => file,
            Err(file) => file,
        };

        let lines = lines_from_file(file);

        let buildlog = lines
            .into_iter()
            .map(|line| strip_ansi(&line))
            .map(|line| format!("   | {}", line))
            .collect::<Vec<String>>()
            .join("\n");

        let total_requirements = require.len();
        let mut missed_requirements: usize = 0;
        let requirements_held: Vec<Result<String, String>> = require
            .into_iter()
            .map(|line| line.to_owned())
            .map(|line| {
                if buildlog.contains(&line) {
                    Ok(line)
                } else {
                    missed_requirements += 1;
                    Err(line)
                }
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
    use std::env;
    use std::path::PathBuf;

    #[test]
    fn test_build_operations() {
        let nix = nix();
        let op = noop(Operation::Build);
        assert_eq!(op.to_string(), "nix-build");

        let ret: Result<fs::File, fs::File> = nix.run(
            nix.safe_command(&op, build_path().as_path(), &["--version"], &[]),
            true,
        );

        assert_run(
            ret,
            Expect::Pass,
            vec!["--no-out-link --keep-going", "--version"],
        );
    }

    #[test]
    fn test_instantiate_operation() {
        let nix = nix();
        let op = noop(Operation::Instantiate);
        assert_eq!(op.to_string(), "nix-instantiate");

        let ret: Result<fs::File, fs::File> = nix.run(
            nix.safe_command(&op, build_path().as_path(), &["--version"], &[]),
            true,
        );

        assert_run(ret, Expect::Pass, vec!["--version"]);
    }

    #[test]
    fn test_query_packages_json() {
        let nix = nix();
        let op = noop(Operation::QueryPackagesJSON);
        assert_eq!(op.to_string(), "nix-env -qa --json");

        let ret: Result<fs::File, fs::File> = nix.run(
            nix.safe_command(&op, build_path().as_path(), &["--version"], &[]),
            true,
        );

        assert_run(
            ret,
            Expect::Pass,
            vec!["--query --available --json", "--version"],
        );
    }

    #[test]
    fn test_query_packages_outputs() {
        let nix = nix();
        let op = noop(Operation::QueryPackagesOutputs);
        assert_eq!(op.to_string(), "nix-env -qaP --no-name --out-path");

        let ret: Result<fs::File, fs::File> = nix.run(
            nix.safe_command(&op, build_path().as_path(), &["--version"], &[]),
            true,
        );

        assert_run(
            ret,
            Expect::Pass,
            vec![
                "--query --available --no-name --attr-path --out-path",
                "--version",
            ],
        );
    }

    #[test]
    fn safe_command_environment() {
        let nix = nix();

        let ret: Result<fs::File, fs::File> = nix.run(
            nix.safe_command::<&OsStr>(&env_noop(), build_path().as_path(), &[], &[]),
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
        let remote = env::var("NIX_REMOTE").unwrap_or("".to_owned());
        let nix = Nix::new(
            "x86_64-linux".to_owned(),
            remote,
            1800,
            Some("4g".to_owned()),
        );

        let ret: Result<fs::File, fs::File> = nix.run(
            nix.safe_command::<&OsStr>(&env_noop(), build_path().as_path(), &[], &[]),
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
        let op = noop(Operation::Build);

        let ret: Result<fs::File, fs::File> = nix.run(
            nix.safe_command::<&OsStr>(&op, build_path().as_path(), &[], &[]),
            true,
        );

        assert_run(
            ret,
            Expect::Pass,
            vec!["--option restrict-eval true", "--option build-timeout 1800"],
        );
    }

    #[test]
    fn set_attrs_nixpkgs() {
        let nix = nix();
        let op = noop(Operation::Build);

        let mut command = nix.safe_command::<&OsStr>(&op, build_path().as_path(), &[], &[]);
        nix.set_attrs_command(
            &mut command,
            File::DefaultNixpkgs,
            vec!["foo".into(), "bar".into()],
        );

        let ret: Result<fs::File, fs::File> = nix.run(command, true);

        assert_run(ret, Expect::Pass, vec!["./default.nix", "-A foo -A bar"]);
    }

    #[test]
    fn set_attrs_nixos() {
        let nix = nix();
        let op = noop(Operation::Instantiate);

        let mut command = nix.safe_command::<&OsStr>(&op, build_path().as_path(), &[], &[]);
        nix.set_attrs_command(
            &mut command,
            File::ReleaseNixOS,
            vec!["foo".into(), "bar".into()],
        );

        let ret: Result<fs::File, fs::File> = nix.run(command, true);

        assert_run(
            ret,
            Expect::Pass,
            vec![
                "./nixos/release.nix",
                "--arg nixpkgs { outPath=./.; revCount=999999; shortRev=\"ofborg\"; }",
            ],
        );
    }

    #[test]
    fn safely_build_attrs_success() {
        let nix = nix();

        let ret: Result<fs::File, fs::File> = nix.safely_build_attrs(
            build_path().as_path(),
            File::DefaultNixpkgs,
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

        let ret: Result<fs::File, fs::File> = nix.safely_build_attrs(
            build_path().as_path(),
            File::DefaultNixpkgs,
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
    fn partition_instantiable_attributes() {
        let nix = nix();

        let ret: (Vec<String>, Vec<(String, Vec<String>)>) = nix
            .safely_partition_instantiable_attrs(
                individual_eval_path().as_path(),
                File::DefaultNixpkgs,
                vec![
                    String::from("fails-instantiation"),
                    String::from("passes-instantiation"),
                    String::from("missing-attr"),
                ],
            );

        assert_eq!(ret.0, vec!["passes-instantiation"]);

        assert_eq!(ret.1[0].0, "fails-instantiation");
        assert_eq!(
            ret.1[0].1[0],
            "trace: You just can't frooble the frozz on this particular system."
        );

        assert_eq!(ret.1[1].0, "missing-attr");
        assert_eq!(
            strip_ansi(&ret.1[1].1[0]),
            "error: attribute 'missing-attr' in selection path 'missing-attr' not found"
        );
    }

    #[test]
    fn safely_instantiate_attrs_failure() {
        let nix = nix();

        let ret: Result<fs::File, fs::File> = nix.safely_instantiate_attrs(
            individual_eval_path().as_path(),
            File::DefaultNixpkgs,
            vec![String::from("fails-instantiation")],
        );

        assert_run(
            ret,
            Expect::Fail,
            vec!["You just can't", "assertion failed"],
        );
    }

    #[test]
    fn safely_instantiate_attrs_success() {
        let nix = nix();

        let ret: Result<fs::File, fs::File> = nix.safely_instantiate_attrs(
            individual_eval_path().as_path(),
            File::DefaultNixpkgs,
            vec![String::from("passes-instantiation")],
        );

        assert_run(ret, Expect::Pass, vec!["-passes-instantiation.drv"]);
    }

    #[test]
    fn safely_evaluate_expr_success() {
        let nix = nix();

        let ret: Result<fs::File, fs::File> = nix.run(
            nix.safely_evaluate_expr_cmd(
                individual_eval_path().as_path(),
                r#"{ foo ? "bar" }: "The magic value is ${foo}""#,
                [("foo", "tux")].iter().cloned().collect(),
                &[],
            ),
            true,
        );

        assert_run(ret, Expect::Pass, vec!["The magic value is tux"]);
    }

    #[test]
    fn strict_sandboxing() {
        let ret: Result<fs::File, fs::File> = nix().safely_build_attrs(
            build_path().as_path(),
            File::DefaultNixpkgs,
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
    fn instantiation_success() {
        let ret: Result<fs::File, fs::File> = nix().safely(
            &Operation::Instantiate,
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

    #[test]
    fn instantiation_nixpkgs_restricted_mode() {
        let ret: Result<fs::File, fs::File> = nix().safely(
            &Operation::Instantiate,
            individual_eval_path().as_path(),
            vec![String::from("-A"), String::from("nixpkgs-restricted-mode")],
            true,
        );

        assert_run(
            ret,
            Expect::Fail,
            vec!["access to path '/fake'", "is forbidden in restricted mode"],
        );
    }
}
