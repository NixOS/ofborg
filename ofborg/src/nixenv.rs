//! Evaluates the expression like Hydra would, with regards to
//! architecture support and recursed packages.
use crate::nix;
use crate::nixstats::EvaluationStats;
use crate::outpathdiff;

use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Read, Seek, SeekFrom, Write};
use std::path::PathBuf;

use tracing::warn;

pub struct HydraNixEnv {
    path: PathBuf,
    nix: nix::Nix,
    check_meta: bool,
}

/// Clone enough of a std::process::Command to be able to display it
/// in debug output upon failure.  The non-Clone fields of Command
/// (e.g. stdin, stdout, stderr) are not cloned, so this is not an
/// `impl Clone`.
fn quasiclone_command(cmd : &std::process::Command) -> std::process::Command {
    let mut c = std::process::Command::new(&cmd.get_program());
    c.args(cmd.get_args());
    c.envs(cmd.get_envs().into_iter().map_while(|(k,vo)| if let Some(v)=vo { Some((k,v)) } else { None }));
    if let Some(dir) = cmd.get_current_dir() { c.current_dir(dir.clone()); };
    c
}

impl HydraNixEnv {
    pub fn new(nix: nix::Nix, path: PathBuf, check_meta: bool) -> HydraNixEnv {
        HydraNixEnv {
            path,
            nix,
            check_meta,
        }
    }

    pub fn execute_with_stats(
        &self,
    ) -> Result<(outpathdiff::PackageOutPaths, EvaluationStats), Error> {
        self.place_nix()?;
        let (failed_command, status, stdout, stderr, stats) = self.run_nix_env();
        self.remove_nix()?;

        if status {
            let outpaths = outpathdiff::parse_lines(&mut BufReader::new(stdout));

            let evaluation_errors = BufReader::new(stderr)
                .lines()
                .collect::<Result<Vec<String>, _>>()?
                .into_iter()
                .filter(|msg| !msg.trim().is_empty())
                .filter(|line| !nix::is_user_setting_warning(line))
                .collect::<Vec<String>>();

            if !evaluation_errors.is_empty() {
                return Err(Error::UncleanEvaluation(evaluation_errors));
            }

            let mut stats = stats.expect("Failed to open stats path, not created?");
            let stats = serde_json::from_reader(&mut stats).map_err(|err| {
                let seek = stats.seek(SeekFrom::Start(0));

                Error::StatsParse(stats, seek, err)
            })?;
            Ok((outpaths, stats))
        } else {
            Err(Error::CommandFailed(failed_command, stderr))
        }
    }

    /// Put outpaths.nix in to the project root, which is what
    /// emulates Hydra's behavior.
    fn place_nix(&self) -> Result<(), Error> {
        let outpath = self.outpath_nix_path();
        let mut file = File::create(&outpath).map_err(|e| Error::CreateFile(outpath, e))?;

        file.write_all(include_bytes!("outpaths.nix"))
            .map_err(|e| Error::WriteFile(file, e))
    }

    fn remove_nix(&self) -> Result<(), Error> {
        let outpath_nix = self.outpath_nix_path();
        let outpath_stats = self.outpath_stats_path();

        fs::remove_file(&outpath_nix).map_err(|e| Error::RemoveFile(outpath_nix, e))?;

        // Removing the stats file can fail if `nix` itself errored, for example
        // when it fails to evaluate something. In this case, we can ignore (but
        // warn about) the error.
        if let Err(e) = fs::remove_file(&outpath_stats) {
            warn!("Failed to remove file {:?}: {:?}", outpath_stats, e)
        }

        Ok(())
    }

    fn outpath_nix_path(&self) -> PathBuf {
        self.path.join(".gc-of-borg-outpaths.nix")
    }

    fn outpath_stats_path(&self) -> PathBuf {
        self.path.join(".gc-of-borg-stats.json")
    }

    fn run_nix_env(&self) -> (std::process::Command, bool, File, File, Result<File, io::Error>) {
        let check_meta = if self.check_meta { "true" } else { "false" };

        let mut cmd = self.nix.safe_command(
            &nix::Operation::QueryPackagesOutputs,
            &self.path,
            &[
                "-f",
                ".gc-of-borg-outpaths.nix",
                "--arg",
                "checkMeta",
                check_meta,
            ],
            &[],
        );
        cmd.env("NIX_SHOW_STATS", "1");
        cmd.env("NIX_SHOW_STATS_PATH", self.outpath_stats_path());

        let quasicloned_cmd = quasiclone_command(&cmd);
        let (status, stdout, stderr) = self.nix.run_stderr_stdout(cmd);
        let stats = File::open(self.outpath_stats_path());

        (quasicloned_cmd, status, stdout, stderr, stats)
    }
}

pub enum Error {
    Io(io::Error),
    CreateFile(PathBuf, io::Error),
    RemoveFile(PathBuf, io::Error),
    WriteFile(File, io::Error),
    CommandFailed(std::process::Command, File),
    StatsParse(File, Result<u64, io::Error>, serde_json::Error),
    UncleanEvaluation(Vec<String>),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::Io(e)
    }
}

impl Error {
    pub fn display(self) -> String {
        match self {
            Error::Io(e) => format!("Failed during the setup of executing nix-env: {:?}", e),
            Error::CreateFile(path, err) => format!("Failed to create file {:?}: {:?}", path, err),
            Error::RemoveFile(path, err) => format!("Failed to remove file {:?}: {:?}", path, err),
            Error::WriteFile(file, err) => {
                format!("Failed to write to file '{:?}': {:?}", file, err)
            }
            Error::CommandFailed(failed_command, mut fd) => {
                let mut buffer = Vec::new();
                let read_result = fd.read_to_end(&mut buffer);
                let bufstr = String::from_utf8_lossy(&buffer);

                match read_result {
                    Ok(_) => format!(concat!("nix-env invocation failed:\n",
                                             "  invocation: {:#?}\n",
                                             "  environment: {:#?}\n",
                                             "{}"),
                                     failed_command.get_args(),
                                     failed_command.get_envs(),
                                     bufstr),
                    Err(e) => format!(
                        "nix-env failed and loading the error result caused a new error {:?}\n\n{}",
                        e, bufstr
                    ),
                }
            }
            Error::UncleanEvaluation(warnings) => {
                format!("nix-env did not evaluate cleanly:\n {:?}", warnings)
            }
            Error::StatsParse(mut fd, seek, parse_err) => {
                let mut buffer = Vec::new();
                let read_result = fd.read_to_end(&mut buffer);
                let bufstr = String::from_utf8_lossy(&buffer);

                let mut lines =
                    String::from("Parsing nix-env's performance statistics failed.\n\n");

                if let Err(seek_err) = seek {
                    lines.push_str(&format!(
                        "Additionally, resetting to the beginning of the output failed with:\n{:?}\n\n",
                        seek_err
                    ));
                }

                if let Err(read_err) = read_result {
                    lines.push_str(&format!(
                        "Additionally, loading the output failed with:\n{:?}\n\n",
                        read_err
                    ));
                }

                lines.push_str(&format!("Parse error:\n{:?}\n\n", parse_err));

                lines.push_str(&format!("Evaluation output:\n{}", bufstr));

                lines
            }
        }
    }
}
