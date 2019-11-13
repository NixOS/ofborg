use crate::nix;
/// Evaluates the expression like Hydra would, with regards to
/// architecture support and recursed packages.
use crate::nixstats::EvaluationStats;
use crate::outpathdiff;
use serde_json;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
use std::path::PathBuf;

pub struct HydraNixEnv {
    path: PathBuf,
    nix: nix::Nix,
    check_meta: bool,
}

impl HydraNixEnv {
    pub fn new(nix: nix::Nix, path: PathBuf, check_meta: bool) -> HydraNixEnv {
        HydraNixEnv {
            nix,
            path,
            check_meta,
        }
    }

    pub fn execute_with_stats(
        &self,
    ) -> Result<(outpathdiff::PackageOutPaths, EvaluationStats), Error> {
        self.place_nix()?;
        let (status, stdout, mut stderr) = self.run_nix_env();
        self.remove_nix()?;

        if status {
            let outpaths = outpathdiff::parse_lines(&mut BufReader::new(stdout));
            let stats = serde_json::from_reader(&mut stderr).map_err(|err| {
                let seek = stderr.seek(SeekFrom::Start(0));

                Error::StatsParse(stderr, seek, err)
            })?;
            Ok((outpaths, stats))
        } else {
            Err(Error::CommandFailed(stderr))
        }
    }

    /// Put outpaths.nix in to the project root, which is what
    /// emulates Hydra's behavior.
    fn place_nix(&self) -> Result<(), std::io::Error> {
        let mut file = File::create(self.outpath_nix_path())?;
        file.write_all(include_bytes!("outpaths.nix"))?;

        Ok(())
    }

    fn remove_nix(&self) -> Result<(), std::io::Error> {
        fs::remove_file(self.outpath_nix_path())?;
        Ok(())
    }

    fn outpath_nix_path(&self) -> PathBuf {
        self.path.join(".gc-of-borg-outpaths.nix")
    }

    fn run_nix_env(&self) -> (bool, File, File) {
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
        self.nix.run_stderr_stdout(cmd)
    }
}

pub enum Error {
    Io(std::io::Error),
    CommandFailed(File),
    StatsParse(File, Result<u64, std::io::Error>, serde_json::Error),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::Io(e)
    }
}

impl Error {
    pub fn display(self) -> String {
        match self {
            Error::Io(e) => format!("Failed during the setup of executing nix-env: {:?}", e),
            Error::CommandFailed(mut fd) => {
                let mut buffer = Vec::new();
                let read_result = fd.read_to_end(&mut buffer);
                let bufstr = String::from_utf8_lossy(&buffer);

                match read_result {
                    Ok(_) => format!("nix-env failed:\n{}", bufstr),
                    Err(e) => format!(
                        "nix-env failed and loading the error result caused a new error {:?}\n\n{}",
                        e, bufstr
                    ),
                }
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
