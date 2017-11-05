use std::path::PathBuf;
use fs2::FileExt;
use std::fs;
use std::io::{Error,ErrorKind};
use std::process::Command;


pub struct Lock {
    lock: Option<fs::File>
}

impl Lock {
    pub fn unlock(&mut self) {
        self.lock = None
    }
}


pub trait GitClonable {
    fn clone_from(&self) -> String;
    fn clone_to(&self) -> PathBuf;
    fn extra_clone_args(&self) -> Vec<String>;

    fn lock_path(&self) -> PathBuf;

    fn lock(&self) -> Result<Lock, Error> {
        let lock = fs::File::create(self.lock_path())?;
        lock.lock_exclusive()?;
        return Ok(Lock{
            lock: Some(lock)
        })
    }

    fn clone_repo(&self) -> Result<(), Error> {
        let mut lock = self.lock()?;

        if self.clone_to().is_dir() {
            return Ok(())
        }

        let result = Command::new("git")
            .arg("clone")
            .args(self.extra_clone_args())
            .arg(&self.clone_from())
            .arg(&self.clone_to())
            .status()?;

        lock.unlock();

        if result.success() {
            return Ok(())
        } else {
            return Err(Error::new(ErrorKind::Other, "Failed to clone"));
        }
    }

    fn fetch_repo(&self) -> Result<(), Error> {
        let mut lock = self.lock()?;

        let result = Command::new("git")
            .arg("fetch")
            .arg("origin")
            .args(self.extra_clone_args())
            .current_dir(self.clone_to())
            .status()?;

        lock.unlock();

        if result.success() {
            return Ok(())
        } else {
            return Err(Error::new(ErrorKind::Other, "Failed to fetch"));
        }
    }
}
