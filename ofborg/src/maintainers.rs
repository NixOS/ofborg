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
    use std::env;
    use std::ffi::OsStr;
    use std::path::{Path, PathBuf};

    fn tpath(component: &str) -> PathBuf {
        return Path::new(env!("CARGO_MANIFEST_DIR")).join(component);
    }

    #[test]
    fn example() {
        let attributes = vec![vec!["kgpg"], vec!["qrencode"], vec!["pass"]];

        let cloner = cached_cloner(&tpath("nixpkgs"));
        let project = cloner.project(
            "commit-msg-list".to_owned(),
            "https://github.com/nixos/nixpkgs.git".to_owned(),
        );

        let working_co = project
            .clone_for("testing-commit-msgs".to_owned(), "123".to_owned())
            .expect("clone should work");

        working_co
            .checkout_origin_ref(OsStr::new("master"))
            .unwrap();

        working_co.fetch_pr(53149).unwrap();

        let paths = working_co.files_changed_from_head("pr").unwrap();
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
            Maintainer::from("yegortimoshenko"),
            vec![Package::from("pkgs.qrencode")],
        );

        assert_eq!(parsed, expect);
    }
}
