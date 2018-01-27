use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct TestScratch {
    root: PathBuf,
}

impl TestScratch {
    pub fn new_dir(ident: &str) -> TestScratch {
        let path = TestScratch {
            root: Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("test-scratch")
                .join("dirs")
                .join(&format!("dir-{}", ident)),
        };
        fs::create_dir_all(path.root.parent().unwrap()).unwrap();

        return path;
    }

    pub fn new_file(ident: &str) -> TestScratch {
        let path = TestScratch {
            root: Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("test-scratch")
                .join("files")
                .join(&format!("file-{}", ident)),
        };
        fs::create_dir_all(path.root.parent().unwrap()).unwrap();

        return path;
    }

    pub fn path(&self) -> PathBuf {
        self.root.clone()
    }
}

impl Drop for TestScratch {
    fn drop(&mut self) {
        Command::new("rm")
            .arg("-rf")
            .arg(self.root.clone())
            .status()
            .expect("cleanup of test-scratch should work");
    }
}
