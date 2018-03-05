extern crate amqp;
extern crate env_logger;

use std::collections::{HashSet, HashMap};
use std::fs::File;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;
use ofborg::nix;
use std::io::Write;

pub struct OutPathDiff {
    calculator: OutPaths,
    pub original: Option<PackageOutPaths>,
    pub current: Option<PackageOutPaths>,
}

impl OutPathDiff {
    pub fn new(nix: nix::Nix, path: PathBuf) -> OutPathDiff {
        OutPathDiff {
            calculator: OutPaths::new(nix, path, false),
            original: None,
            current: None,
        }
    }

    pub fn find_before(&mut self) -> Result<bool, File> {
        let x = self.run();
        match x {
            Ok(f) => {
                self.original = Some(f);
                return Ok(true);
            }
            Err(e) => {
                info!("Failed to find Before list");
                return Err(e);
            }
        }
    }

    pub fn find_after(&mut self) -> Result<bool, File> {
        if self.original == None {
            debug!("Before is None, not bothering with After");
            return Ok(false);
        }

        let x = self.run();
        match x {
            Ok(f) => {
                self.current = Some(f);
                return Ok(true);
            }
            Err(e) => {
                info!("Failed to find After list");
                return Err(e);
            }
        }
    }

    pub fn package_diff(&self) -> Option<(Vec<PackageArch>, Vec<PackageArch>)> {
        if let Some(ref cur) = self.current {
            if let Some(ref orig) = self.original {
                let orig_set: HashSet<&PackageArch> = orig.keys().collect();
                let cur_set: HashSet<&PackageArch> = cur.keys().collect();

                let removed: Vec<PackageArch> = orig_set.difference(&cur_set).map(|ref p| (**p).clone()).collect();
                let added: Vec<PackageArch> = cur_set.difference(&orig_set).map(|ref p| (**p).clone()).collect();
                return Some((removed, added));
            } else {
                return None;
            }
        } else {
            return None;
        }
    }

    pub fn calculate_rebuild(self) -> Option<Vec<PackageArch>> {
        let mut rebuild: Vec<PackageArch> = vec![];

        if let Some(cur) = self.current {
            if let Some(orig) = self.original {
                for key in cur.keys() {
                    trace!("Checking out {:?}", key);
                    if cur.get(key) != orig.get(key) {
                        trace!("    {:?} != {:?}", cur.get(key), orig.get(key));
                        rebuild.push(key.clone())
                    } else {
                        trace!("    {:?} == {:?}", cur.get(key), orig.get(key));
                    }
                }

                return Some(rebuild);
            }
        }

        return None;
    }

    fn run(&mut self) -> Result<PackageOutPaths, File> {
        self.calculator.find()
    }
}

type PackageOutPaths = HashMap<PackageArch, OutPath>;

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
pub struct PackageArch {
    pub package: Package,
    pub architecture: Architecture,
}
type Package = String;
type Architecture = String;
type OutPath = String;

pub struct OutPaths {
    path: PathBuf,
    nix: nix::Nix,
    check_meta: bool,
}

impl OutPaths {
    pub fn new(nix: nix::Nix, path: PathBuf, check_meta: bool) -> OutPaths {
        OutPaths {
            nix: nix,
            path: path,
            check_meta: check_meta,
        }
    }

    pub fn find(&self) -> Result<PackageOutPaths, File> {
        self.run()
    }

    fn run(&self) -> Result<PackageOutPaths, File> {
        self.place_nix();
        let ret = self.execute();
        self.remove_nix();

        match ret {
            Ok(file) => Ok(parse_lines(&mut BufReader::new(file))),
            Err(e) => Err(e),
        }
    }

    fn place_nix(&self) {
        let mut file = File::create(self.nix_path()).expect("Failed to create nix out path check");
        file.write_all(include_str!("outpaths.nix").as_bytes())
            .expect("Failed to place outpaths.nix");
    }

    fn remove_nix(&self) {
        fs::remove_file(self.nix_path()).expect("Failed to delete outpaths.nix");
    }

    fn nix_path(&self) -> PathBuf {
        let mut dest = self.path.clone();
        dest.push(".gc-of-borg-outpaths.nix");

        dest
    }

    fn execute(&self) -> Result<File, File> {
        let check_meta: String;

        if self.check_meta {
            check_meta = String::from("true");
        } else {
            check_meta = String::from("false");
        }

        self.nix.safely(
            nix::Operation::QueryPackagesOutputs,
            &self.path,
            vec![
                String::from("-f"),
                String::from(".gc-of-borg-outpaths.nix"),
                String::from("--arg"),
                String::from("checkMeta"),
                check_meta,
            ],
            true,
        )
    }
}


fn parse_lines(data: &mut BufRead) -> PackageOutPaths {
    data.lines()
        .filter_map(|line| match line {
            Ok(line) => Some(line),
            Err(_) => None,
        })
        .filter_map(|line| {
            let split: Vec<&str> = line.split_whitespace().collect();
            if split.len() == 2 {
                let outpaths = String::from(split[1]);

                let path: Vec<&str> = split[0].rsplitn(2, ".").collect();
                if path.len() == 2 {
                    Some((
                        PackageArch {
                            package: String::from(path[1]),
                            architecture: String::from(path[0]),
                        },
                        outpaths,
                    ))
                } else {
                    info!("Warning: Didn't detect an architecture for {:?}", path);
                    None
                }
            } else {
                info!("Warning: not 2 word segments in {:?}", split);
                None
            }

        })
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::io::Cursor;

    const TEST_LINES: &'static str = "
kindlegen.x86_64-darwin                                                    /nix/store/sgabv7byhan6b0rjspd3p1bd7yw91f30-kindlegen-2.9
python27Packages.pyinotify.i686-linux                                      /nix/store/rba0hbq6i4camvhpj9723dvs4b511ryn-python2.7-pyinotify-0.9.6
pan.i686-linux                                                             /nix/store/6djnw9s2z5iy0c741qa8yk0k2v6bxrra-pan-0.139
gnome3.evolution_data_server.aarch64-linux                                 /nix/store/fmxf25kyxb62v9arc64fypb2ilxifsh0-evolution-data-server-3.26.3
";

    #[test]
    fn test_parse_outputs() {
        let mut expect: PackageOutPaths = HashMap::new();
        expect.insert(
            PackageArch {
                package: "kindlegen".to_owned(),
                architecture: "x86_64-darwin".to_owned(),
            },
            "/nix/store/sgabv7byhan6b0rjspd3p1bd7yw91f30-kindlegen-2.9".to_owned(),
        );

        expect.insert(
            PackageArch {
                architecture: "aarch64-linux".to_owned(),
                package: "gnome3.evolution_data_server".to_owned(),
            },
            "/nix/store/fmxf25kyxb62v9arc64fypb2ilxifsh0-evolution-data-server-3.26.3".to_owned(),
        );

        expect.insert(
            PackageArch {
                architecture: "i686-linux".to_owned(),
                package: "python27Packages.pyinotify".to_owned(),
            },
            "/nix/store/rba0hbq6i4camvhpj9723dvs4b511ryn-python2.7-pyinotify-0.9.6".to_owned(),
        );

        expect.insert(
            PackageArch {
                architecture: "i686-linux".to_owned(),
                package: "pan".to_owned(),
            },
            "/nix/store/6djnw9s2z5iy0c741qa8yk0k2v6bxrra-pan-0.139".to_owned(),
        );
        assert_eq!(parse_lines(&mut Cursor::new(TEST_LINES)), expect);
    }

}
