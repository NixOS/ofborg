use std::collections::HashMap;

#[derive(Deserialize, Debug, Eq, PartialEq)]
struct ImpactedMaintainers(HashMap<Maintainer, Vec<Package>>);
#[derive(Deserialize, Debug, Eq, PartialEq, Hash)]
struct Maintainer(String);
impl<'a> From<&'a str> for Maintainer {
    fn from(name: &'a str) -> Maintainer {
        Maintainer(name.to_owned())
    }
}
#[derive(Deserialize, Debug, Eq, PartialEq, Hash)]
struct Package(String);
impl<'a> From<&'a str> for Package {
    fn from(name: &'a str) -> Package {
        Package(name.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use checkout::cached_cloner;
    use clone::GitClonable;
    use ofborg::nix::Nix;
    use ofborg::test_scratch::TestScratch;
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

        let pathstr = serde_json::to_string(&paths).unwrap();
        let attrstr = serde_json::to_string(&attributes).unwrap();

        let mut argstrs: HashMap<&str, &str> = HashMap::new();
        argstrs.insert("changedattrsjson", &attrstr);
        argstrs.insert("changedpathsjson", &pathstr);

        let remote = env::var("NIX_REMOTE").unwrap_or("".to_owned());
        let nix = Nix::new("x86_64-linux".to_owned(), remote, 1800, None);
        let ret = nix
            .safely_evaluate_expr_cmd(
                &working_co.clone_to(),
                include_str!("./maintainers.nix"),
                argstrs,
            )
            .output()
            .expect(":)");

        let parsed: ImpactedMaintainers =
            serde_json::from_str(&String::from_utf8(ret.stdout).unwrap()).unwrap();

        let mut expect = ImpactedMaintainers(HashMap::new());
        expect.0.insert(
            Maintainer::from("test"),
            vec![Package::from("pkgs.foo.bar.packageA")],
        );

        assert_eq!(parsed, expect);
    }
}
