use fs2::FileExt;
use std::ffi::OsStr;
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use std::process::Command;

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
        info!("Locking {:?}", self.lock_path());

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

        info!("git am --abort");
        Command::new("git")
            .arg("am")
            .arg("--abort")
            .current_dir(self.clone_to())
            .status()?;

        info!("git merge --abort");
        Command::new("git")
            .arg("merge")
            .arg("--abort")
            .current_dir(self.clone_to())
            .status()?;

        info!("git reset --hard");
        Command::new("git")
            .arg("reset")
            .arg("--hard")
            .current_dir(self.clone_to())
            .status()?;

        lock.unlock();

        Ok(())
    }

    fn checkout(&self, git_ref: &OsStr) -> Result<(), Error> {
        let mut lock = self.lock()?;

        debug!("git checkout {:?}", git_ref);

        let result = Command::new("git")
            .arg("checkout")
            .arg(git_ref)
            .current_dir(self.clone_to())
            .status()?;

        lock.unlock();

        if result.success() {
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, "Failed to checkout"))
        }
    }
}
