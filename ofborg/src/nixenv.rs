/// Evaluates the expression like Hydra would, with regards to
/// architecture support and recursed packages.
use crate::nixstats::EvaluationStats;
use crate::outpathdiff;
use ofborg::nix;
use serde_json;
use std::fs;
use std::fs::File;
use std::io::BufReader;
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

    pub fn execute(&self) -> Result<(outpathdiff::PackageOutPaths, EvaluationStats), Error> {
        self.place_nix()?;
        let (status, stdout, mut stderr) = self.run_nix_env();
        self.remove_nix()?;

        if status {
            Err(Error::Fd(stderr))
        } else if let Ok(stats) = serde_json::from_reader(&mut stderr) {
            let outpaths = outpathdiff::parse_lines(&mut BufReader::new(stdout));
            Ok((outpaths, stats))
        } else {
            stderr
                .seek(SeekFrom::Start(0))
                .expect("Seeking to Start(0)");
            Err(Error::Fd(stderr))
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
    Fd(File),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::Io(e)
    }
}
