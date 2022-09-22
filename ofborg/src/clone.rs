use fs2::FileExt;

use std::ffi::OsStr;
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use std::process::{Command, Stdio};

use tracing::{debug, info, warn};

pub struct Lock {
    lock: Option<fs::File>,
}

impl Lock {
    pub fn unlock(&mut self) {
        self.lock = None
    }
}

pub trait GitClonable {
    fn clone_from(&self) -> String;
    fn clone_to(&self) -> PathBuf;
    fn extra_clone_args(&self) -> Vec<&OsStr>;

    fn lock_path(&self) -> PathBuf;

    fn lock(&self) -> Result<Lock, Error> {
        debug!("Locking {:?}", self.lock_path());

        match fs::File::create(self.lock_path()) {
            Err(e) => {
                warn!("Failed to create lock file {:?}: {}", self.lock_path(), e);
                Err(e)
            }
            Ok(lock) => match lock.lock_exclusive() {
                Err(e) => {
                    warn!(
                        "Failed to get exclusive lock on file {:?}: {}",
                        self.lock_path(),
                        e
                    );
                    Err(e)
                }
                Ok(_) => {
                    debug!("Got lock on {:?}", self.lock_path());
                    Ok(Lock { lock: Some(lock) })
                }
            },
        }
    }

    fn clone_repo(&self) -> Result<(), Error> {
        let mut lock = self.lock()?;

        if self.clone_to().is_dir() {
            debug!("Found dir at {:?}, initial clone is done", self.clone_to());
            return Ok(());
        }

        info!(
            "Initial cloning of {} to {:?}",
            self.clone_from(),
            self.clone_to()
        );

        let result = Command::new("git")
            .arg("clone")
            .args(self.extra_clone_args())
            .arg(&self.clone_from())
            .arg(&self.clone_to())
            .stdout(Stdio::null())
            .status()?;

        lock.unlock();

        if result.success() {
            Ok(())
        } else {
            Err(Error::new(
                ErrorKind::Other,
                format!(
                    "Failed to clone from {:?} to {:?}",
                    self.clone_from(),
                    self.clone_to()
                ),
            ))
        }
    }

    fn fetch_repo(&self) -> Result<(), Error> {
        let mut lock = self.lock()?;

        info!("Fetching from origin in {:?}", self.clone_to());
        let result = Command::new("git")
            .arg("fetch")
            .arg("origin")
            .current_dir(self.clone_to())
            .stdout(Stdio::null())
            .status()?;

        lock.unlock();

        if result.success() {
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, "Failed to fetch"))
        }
    }

    fn clean(&self) -> Result<(), Error> {
        let mut lock = self.lock()?;

        debug!("git am --abort");
        Command::new("git")
            .arg("am")
            .arg("--abort")
            .current_dir(self.clone_to())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()?;

        debug!("git merge --abort");
        Command::new("git")
            .arg("merge")
            .arg("--abort")
            .current_dir(self.clone_to())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()?;

        debug!("git reset --hard");
        Command::new("git")
            .arg("reset")
            .arg("--hard")
            .current_dir(self.clone_to())
            .stdout(Stdio::null())
            .status()?;

        lock.unlock();

        Ok(())
    }

    fn checkout(&self, git_ref: &OsStr) -> Result<(), Error> {
        let mut lock = self.lock()?;
        let current_dir = self.clone_to();

        debug!("git checkout {:?}", git_ref);
        let git_checkout = || {
            Command::new("git")
                .arg("checkout")
                .arg(git_ref)
                .current_dir(&current_dir)
                .stdout(Stdio::null())
                .status()
        };
        let result = git_checkout()?;

        if result.success() {
            lock.unlock();
            Ok(())
        } else {
            warn!(
                "failed to checkout {:?}, attempting to clean up checkout",
                git_ref
            );

            // see if cleaning up the checkout will fix the issue
            debug!("git clean -dfx in {:?}", &current_dir);
            Command::new("git")
                .args(&["clean", "-dfx"])
                .current_dir(&current_dir)
                .stdout(Stdio::null())
                .status()?;

            // try again, in case it was just an unclean checkout causing issues
            debug!("git checkout attempt two {:?}", git_ref);
            let result = git_checkout()?;

            if result.success() {
                lock.unlock();
                Ok(())
            } else {
                Err(Error::new(ErrorKind::Other, "Failed to checkout"))
            }
        }
    }
}
