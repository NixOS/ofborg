use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use tracing::debug;

pub struct TestScratch {
    root: PathBuf,
}

impl TestScratch {
    pub fn new_dir(ident: &str) -> TestScratch {
        let scratch = TestScratch {
            root: Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("test-scratch")
                .join("dirs")
                .join(format!("dir-{ident}")),
        };

        TestScratch::create_dir(&scratch);

        scratch
    }

    pub fn new_file(ident: &str) -> TestScratch {
        let scratch = TestScratch {
            root: Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("test-scratch")
                .join("files")
                .join(format!("file-{ident}")),
        };

        TestScratch::create_dir(&scratch);
        scratch
    }

    fn create_dir(path: &TestScratch) {
        let target = path.root.parent().unwrap();
        debug!("Creating directory {target:?}");
        fs::create_dir_all(target).unwrap();
    }

    pub fn path(&self) -> PathBuf {
        self.root.clone()
    }

    pub fn string(&self) -> String {
        self.path().to_str().unwrap().to_owned()
    }
}

impl Drop for TestScratch {
    fn drop(&mut self) {
        debug!("Deleting root {:?}", self.root);
        Command::new("rm")
            .arg("-rf")
            .arg(self.root.clone())
            .status()
            .expect("cleanup of test-scratch should work");
    }
}
