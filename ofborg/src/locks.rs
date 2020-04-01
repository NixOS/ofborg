use fs2::FileExt;

use std::fs;
use std::io::Error;
use std::path::PathBuf;

pub trait Lockable {
    fn lock_path(&self) -> PathBuf;

    fn lock(&self) -> Result<Lock, Error> {
        let lock = fs::File::create(self.lock_path())?;
        lock.lock_exclusive()?;
        Ok(Lock { lock: Some(lock) })
    }
}

pub struct Lock {
    lock: Option<fs::File>,
}

impl Lock {
    pub fn unlock(&mut self) {
        self.lock = None
    }
}
