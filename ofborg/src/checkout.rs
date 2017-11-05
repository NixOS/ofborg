use std::path::{Path,PathBuf};
use md5;
use std::fs;
use std::io::Error;
use ofborg::clone;
use ofborg::clone::GitClonable;

pub struct CachedCloner {
    root: PathBuf
}

pub fn cached_cloner(path: &Path) -> CachedCloner {
    return CachedCloner{
        root: path.to_path_buf()
    }
}

pub struct CachedProject {
    root: PathBuf,
    clone_url: String
}

impl CachedCloner {
    pub fn project(&self, name: String, clone_url: String) -> CachedProject {
        // <root>/repo/<hash>/clone
        // <root>/repo/<hash>/clone.lock
        // <root>/repo/<hash>/<type>/<id>
        // <root>/repo/<hash>/<type>/<id>.lock

        let mut new_root = self.root.clone();
        new_root.push("repo");
        new_root.push(format!("{:x}", md5::compute(&name)));

        return CachedProject{
            root: new_root,
            clone_url: clone_url
        }
    }
}

impl CachedProject {
    pub fn checkout_ref(&self,
                        name: String,
                        git_ref: String,
    ) -> Result<String, Error> {
        let repo_cache_path = self.prefetch_cache()?;

        // let build_dir = self.build_dir();

        return Ok(repo_cache_path.to_str().unwrap().to_string())
    }

    fn prefetch_cache(&self) -> Result<PathBuf, Error> {
        fs::create_dir_all(&self.root)?;

        self.clone_repo()?;

        return Ok(self.clone_to());
    }
}

impl clone::GitClonable for CachedProject {

    fn clone_from(&self) -> String {
        return self.clone_url.clone()
    }

    fn clone_to(&self) -> PathBuf {
        let mut clone_path = self.root.clone();
        clone_path.push("clone");
        return clone_path
    }

    fn lock_path(&self) -> PathBuf {
        let mut clone_path = self.root.clone();
        clone_path.set_file_name("clone.lock");
        return clone_path
    }

    fn extra_clone_args(&self) -> Vec<String> {
        return vec!()
    }
}

/*
    fn try_clone_repo(&self) -> Result<(), Error> {


        let result = Command::new("git")
            .arg("clone")
            .arg("--bare")
            .arg(&self.clone_url)
            .arg(&self.clone_path())
            .status()?;

        if result.success() {
            return Ok(())
        } else {
            return Err(Error::new(ErrorKind::Other, "Failed to clone"));
        }
    }

    fn try_fetch(&self) -> Result<(), Error> {
        let result = Command::new("git")
            .arg("fetch")
            .arg("origin")
            .current_dir(self.clone_path())
            .status()?;

        if result.success() {
            return Ok(())
        } else {
            return Err(Error::new(ErrorKind::Other, "Failed to fetch"));
        }
    }


}

impl locks::Lockable for CachedProject {
    fn lock_path(&self) -> PathBuf {
        let mut clone_path = self.root.clone();
        clone_path.set_file_name("clone.lock");
        return clone_path
    }
}
*/
