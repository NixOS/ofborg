use crate::clone::{self, GitClonable};

use std::ffi::{OsStr, OsString};
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use tracing::info;

pub struct CachedCloner {
    root: PathBuf,
}

pub fn cached_cloner(path: &Path) -> CachedCloner {
    CachedCloner {
        root: path.to_path_buf(),
    }
}

pub struct CachedProject {
    root: PathBuf,
    clone_url: String,
}

pub struct CachedProjectCo {
    root: PathBuf,
    id: String,
    clone_url: String,
    local_reference: PathBuf,
}

impl CachedCloner {
    pub fn project(&self, name: &str, clone_url: String) -> CachedProject {
        // <root>/repo/<hash>/clone
        // <root>/repo/<hash>/clone.lock
        // <root>/repo/<hash>/<type>/<id>
        // <root>/repo/<hash>/<type>/<id>.lock

        let mut new_root = self.root.clone();
        new_root.push("repo");
        new_root.push(format!("{:x}", md5::compute(name)));

        CachedProject {
            root: new_root,
            clone_url,
        }
    }
}

impl CachedProject {
    pub fn clone_for(&self, use_category: String, id: String) -> Result<CachedProjectCo, Error> {
        self.prefetch_cache()?;

        let mut new_root = self.root.clone();
        new_root.push(use_category);

        Ok(CachedProjectCo {
            root: new_root,
            id,
            clone_url: self.clone_from(),
            local_reference: self.clone_to(),
        })
    }

    fn prefetch_cache(&self) -> Result<PathBuf, Error> {
        fs::create_dir_all(&self.root)?;

        self.clone_repo()?;
        self.fetch_repo()?;

        Ok(self.clone_to())
    }
}

impl CachedProjectCo {
    pub fn checkout_origin_ref(&self, git_ref: &OsStr) -> Result<String, Error> {
        let mut pref = OsString::from("origin/");
        pref.push(git_ref);

        self.checkout_ref(&pref)
    }

    pub fn checkout_ref(&self, git_ref: &OsStr) -> Result<String, Error> {
        fs::create_dir_all(&self.root)?;

        self.clone_repo()?;
        self.fetch_repo()?;
        self.clean()?;
        self.checkout(git_ref)?;

        // let build_dir = self.build_dir();

        Ok(self.clone_to().to_str().unwrap().to_string())
    }

    pub fn fetch_pr(&self, pr_id: u64) -> Result<(), Error> {
        let mut lock = self.lock()?;

        info!("Fetching PR #{}", pr_id);
        let result = Command::new("git")
            .arg("fetch")
            .arg("origin")
            .arg(format!("+refs/pull/{}/head:pr", pr_id))
            .current_dir(self.clone_to())
            .stdout(Stdio::null())
            .status()?;

        lock.unlock();

        if result.success() {
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, "Failed to fetch PR"))
        }
    }

    pub fn commit_exists(&self, commit: &OsStr) -> bool {
        let mut lock = self.lock().expect("Failed to lock");

        info!("Checking if commit {:?} exists", commit);
        let result = Command::new("git")
            .arg("--no-pager")
            .arg("show")
            .arg(commit)
            .current_dir(self.clone_to())
            .stdout(Stdio::null())
            .status()
            .expect("git show <commit> failed");

        lock.unlock();

        result.success()
    }

    pub fn merge_commit(&self, commit: &OsStr) -> Result<(), Error> {
        let mut lock = self.lock()?;

        info!("Merging commit {:?}", commit);
        let result = Command::new("git")
            .arg("merge")
            .arg("--no-gpg-sign")
            .arg("-m")
            .arg("Automatic merge for GrahamCOfBorg")
            .arg(commit)
            .current_dir(self.clone_to())
            .stdout(Stdio::null())
            .status()?;

        lock.unlock();

        if result.success() {
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, "Failed to merge"))
        }
    }

    pub fn commit_messages_from_head(&self, commit: &str) -> Result<Vec<String>, Error> {
        let mut lock = self.lock()?;

        let result = Command::new("git")
            .arg("log")
            .arg("--format=format:%s")
            .arg(format!("HEAD..{}", commit))
            .current_dir(self.clone_to())
            .output()?;

        lock.unlock();

        if result.status.success() {
            Ok(String::from_utf8_lossy(&result.stdout)
                .lines()
                .map(|l| l.to_owned())
                .collect())
        } else {
            Err(Error::new(
                ErrorKind::Other,
                String::from_utf8_lossy(&result.stderr).to_lowercase(),
            ))
        }
    }

    pub fn files_changed_from_head(&self, commit: &str) -> Result<Vec<String>, Error> {
        let mut lock = self.lock()?;

        let result = Command::new("git")
            .arg("diff")
            .arg("--name-only")
            .arg(format!("HEAD...{}", commit))
            .current_dir(self.clone_to())
            .output()?;

        lock.unlock();

        if result.status.success() {
            Ok(String::from_utf8_lossy(&result.stdout)
                .lines()
                .map(|l| l.to_owned())
                .collect())
        } else {
            Err(Error::new(
                ErrorKind::Other,
                String::from_utf8_lossy(&result.stderr).to_lowercase(),
            ))
        }
    }
}

impl clone::GitClonable for CachedProjectCo {
    fn clone_from(&self) -> String {
        self.clone_url.clone()
    }

    fn clone_to(&self) -> PathBuf {
        let mut clone_path = self.root.clone();
        clone_path.push(&self.id);
        clone_path
    }

    fn lock_path(&self) -> PathBuf {
        let mut lock_path = self.root.clone();
        lock_path.push(format!("{}.lock", self.id));
        lock_path
    }

    fn extra_clone_args(&self) -> Vec<&OsStr> {
        let local_ref = self.local_reference.as_ref();
        vec![
            OsStr::new("--shared"),
            OsStr::new("--reference-if-able"),
            local_ref,
        ]
    }
}

impl clone::GitClonable for CachedProject {
    fn clone_from(&self) -> String {
        self.clone_url.clone()
    }

    fn clone_to(&self) -> PathBuf {
        let mut clone_path = self.root.clone();
        clone_path.push("clone");
        clone_path
    }

    fn lock_path(&self) -> PathBuf {
        let mut clone_path = self.root.clone();
        clone_path.push("clone.lock");
        clone_path
    }

    fn extra_clone_args(&self) -> Vec<&OsStr> {
        vec![OsStr::new("--bare")]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_scratch::TestScratch;
    use std::path::{Path, PathBuf};
    use std::process::{Command, Stdio};

    fn tpath(component: &str) -> PathBuf {
        return Path::new(env!("CARGO_MANIFEST_DIR")).join(component);
    }

    fn make_pr_repo(bare: &Path, co: &Path) -> String {
        let output = Command::new("bash")
            .current_dir(tpath("./test-srcs"))
            .arg("./make-pr.sh")
            .arg(bare)
            .arg(co)
            .stdout(Stdio::piped())
            .output()
            .expect("building the test PR failed");

        let stderr =
            String::from_utf8(output.stderr).unwrap_or_else(|err| format!("warning: {}", err));
        println!("{}", stderr);

        let hash = String::from_utf8(output.stdout).expect("Should just be a hash");
        return hash.trim().to_owned();
    }

    #[test]
    pub fn test_commit_msg_list() {
        let workingdir = TestScratch::new_dir("test-test-commit-msg-list");

        let bare = TestScratch::new_dir("bare-commit-messages");
        let mk_co = TestScratch::new_dir("mk-commit-messages");
        let hash = make_pr_repo(&bare.path(), &mk_co.path());

        let cloner = cached_cloner(&workingdir.path());
        let project = cloner.project("commit-msg-list", bare.string());
        let working_co = project
            .clone_for("testing-commit-msgs".to_owned(), "123".to_owned())
            .expect("clone should work");
        working_co
            .checkout_origin_ref(OsStr::new("master"))
            .unwrap();

        let expect: Vec<String> = vec!["check out this cool PR".to_owned()];

        assert_eq!(
            working_co
                .commit_messages_from_head(&hash)
                .expect("fetching messages should work",),
            expect
        );
    }

    #[test]
    pub fn test_files_changed_list() {
        let workingdir = TestScratch::new_dir("test-test-files-changed-list");

        let bare = TestScratch::new_dir("bare-files-changed");
        let mk_co = TestScratch::new_dir("mk-files-changed");
        let hash = make_pr_repo(&bare.path(), &mk_co.path());

        let cloner = cached_cloner(&workingdir.path());
        let project = cloner.project("commit-files-changed-list", bare.string());
        let working_co = project
            .clone_for("testing-files-changed".to_owned(), "123".to_owned())
            .expect("clone should work");
        working_co
            .checkout_origin_ref(OsStr::new("master"))
            .unwrap();

        let expect: Vec<String> = vec!["default.nix".to_owned(), "hi another file".to_owned()];

        assert_eq!(
            working_co
                .files_changed_from_head(&hash)
                .expect("fetching files changed should work",),
            expect
        );
    }
}
