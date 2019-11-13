use crate::nix::Nix;
use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::path::Path;
use tempfile::NamedTempFile;

#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct ImpactedMaintainers(HashMap<Maintainer, Vec<Package>>);
pub struct MaintainersByPackage(pub HashMap<Package, HashSet<Maintainer>>);

#[derive(Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Maintainer(String);
impl<'a> From<&'a str> for Maintainer {
    fn from(name: &'a str) -> Maintainer {
        Maintainer(name.to_ascii_lowercase().to_owned())
    }
}
#[derive(Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Package(String);
impl<'a> From<&'a str> for Package {
    fn from(name: &'a str) -> Package {
        Package(name.to_owned())
    }
}

#[derive(Debug)]
pub enum CalculationError {
    DeserializeError(serde_json::Error),
    Io(std::io::Error),
    Utf8(std::string::FromUtf8Error),
}
impl From<serde_json::Error> for CalculationError {
    fn from(e: serde_json::Error) -> CalculationError {
        CalculationError::DeserializeError(e)
    }
}
impl From<std::io::Error> for CalculationError {
    fn from(e: std::io::Error) -> CalculationError {
        CalculationError::Io(e)
    }
}
impl From<std::string::FromUtf8Error> for CalculationError {
    fn from(e: std::string::FromUtf8Error) -> CalculationError {
        CalculationError::Utf8(e)
    }
}

impl ImpactedMaintainers {
    pub fn calculate(
        nix: &Nix,
        checkout: &Path,
        paths: &[String],
        attributes: &[Vec<&str>],
    ) -> Result<ImpactedMaintainers, CalculationError> {
        let mut path_file = NamedTempFile::new()?;
        let pathstr = serde_json::to_string(&paths)?;
        write!(path_file, "{}", pathstr)?;

        let mut attr_file = NamedTempFile::new()?;
        let attrstr = serde_json::to_string(&attributes)?;
        write!(attr_file, "{}", attrstr)?;

        let mut argstrs: HashMap<&str, &str> = HashMap::new();
        argstrs.insert("changedattrsjson", attr_file.path().to_str().unwrap());
        argstrs.insert("changedpathsjson", path_file.path().to_str().unwrap());

        let mut cmd = nix.safely_evaluate_expr_cmd(
            &checkout,
            include_str!("./maintainers.nix"),
            argstrs,
            &[path_file.path(), attr_file.path()],
        );

        let ret = cmd.output()?;

        Ok(serde_json::from_str(&String::from_utf8(ret.stdout)?)?)
    }

    pub fn maintainers(&self) -> Vec<String> {
        self.0
            .iter()
            .map(|(maintainer, _)| maintainer.0.clone())
            .collect()
    }

    pub fn maintainers_by_package(&self) -> MaintainersByPackage {
        let mut bypkg = MaintainersByPackage(HashMap::new());

        for (maintainer, packages) in self.0.iter() {
            for package in packages.iter() {
                bypkg
                    .0
                    .entry(package.clone())
                    .or_insert_with(HashSet::new)
                    .insert(maintainer.clone());
            }
        }

        bypkg
    }
}

impl std::fmt::Display for ImpactedMaintainers {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let d = self
            .0
            .iter()
            .map(|(maintainer, packages)| {
                format!(
                    "{}: {}",
                    maintainer.0,
                    packages
                        .iter()
                        .map(|pkg| pkg.0.clone())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            })
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", d)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::checkout::cached_cloner;
    use crate::clone::GitClonable;
    use crate::test_scratch::TestScratch;
    use std::env;
    use std::ffi::OsStr;
    use std::path::{Path, PathBuf};
    use std::process::Command;
    use std::process::Stdio;

    fn tpath(component: &str) -> PathBuf {
        return Path::new(env!("CARGO_MANIFEST_DIR")).join(component);
    }

    fn make_pr_repo(bare: &Path, co: &Path) -> String {
        let output = Command::new("./make-maintainer-pr.sh")
            .current_dir(tpath("./test-srcs"))
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
    fn example() {
        let workingdir = TestScratch::new_dir("test-maintainers-example");

        let bare = TestScratch::new_dir("test-maintainers-example-bare");
        let mk_co = TestScratch::new_dir("test-maintainers-example-co");
        let hash = make_pr_repo(&bare.path(), &mk_co.path());

        let attributes = vec![vec!["foo", "bar", "packageA"]];

        let cloner = cached_cloner(&workingdir.path());
        let project = cloner.project("maintainer-test", bare.string());

        let working_co = project
            .clone_for("testing-maintainer-list".to_owned(), "123".to_owned())
            .expect("clone should work");

        working_co
            .checkout_origin_ref(&OsStr::new("master"))
            .unwrap();

        let paths = working_co.files_changed_from_head(&hash).unwrap();

        working_co.checkout_ref(&OsStr::new(&hash)).unwrap();

        let remote = env::var("NIX_REMOTE").unwrap_or("".to_owned());
        let nix = Nix::new("x86_64-linux".to_owned(), remote, 1800, None);

        let parsed =
            ImpactedMaintainers::calculate(&nix, &working_co.clone_to(), &paths, &attributes);

        let mut expect = ImpactedMaintainers(HashMap::new());
        expect.0.insert(
            Maintainer::from("test"),
            vec![Package::from("foo.bar.packageA")],
        );

        assert_eq!(parsed.unwrap(), expect);
    }
}
