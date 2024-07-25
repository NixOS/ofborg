use crate::maintainers::{Maintainer, MaintainersByPackage};
use crate::outpathdiff::PackageArch;
use crate::tasks;

use tracing::info;

pub struct StdenvTagger {
    possible: Vec<String>,
    selected: Vec<String>,
}

impl Default for StdenvTagger {
    fn default() -> StdenvTagger {
        let mut t = StdenvTagger {
            possible: vec![
                String::from("10.rebuild-linux-stdenv"),
                String::from("10.rebuild-darwin-stdenv"),
            ],
            selected: vec![],
        };
        t.possible.sort();

        t
    }
}

impl StdenvTagger {
    pub fn new() -> StdenvTagger {
        Default::default()
    }

    pub fn changed(&mut self, systems: Vec<tasks::eval::stdenvs::System>) {
        for system in systems {
            match system {
                tasks::eval::stdenvs::System::X8664Darwin => {
                    self.selected.push(String::from("10.rebuild-darwin-stdenv"));
                }
                tasks::eval::stdenvs::System::X8664Linux => {
                    self.selected.push(String::from("10.rebuild-linux-stdenv"));
                }
            }
        }

        for tag in &self.selected {
            if !self.possible.contains(tag) {
                panic!(
                    "Tried to add label {} but it isn't in the possible list!",
                    tag
                );
            }
        }
    }

    pub fn tags_to_add(&self) -> Vec<String> {
        self.selected.clone()
    }

    pub fn tags_to_remove(&self) -> Vec<String> {
        let mut remove = self.possible.clone();
        for tag in &self.selected {
            let pos = remove.binary_search(tag).unwrap();
            remove.remove(pos);
        }

        remove
    }
}

pub struct PkgsAddedRemovedTagger {
    possible: Vec<String>,
    selected: Vec<String>,
}

impl Default for PkgsAddedRemovedTagger {
    fn default() -> PkgsAddedRemovedTagger {
        let mut t = PkgsAddedRemovedTagger {
            possible: vec![
                String::from("8.has: package (new)"),
                String::from("8.has: clean-up"),
            ],
            selected: vec![],
        };
        t.possible.sort();

        t
    }
}

impl PkgsAddedRemovedTagger {
    pub fn new() -> PkgsAddedRemovedTagger {
        Default::default()
    }

    pub fn changed(&mut self, removed: &[PackageArch], added: &[PackageArch]) {
        if !removed.is_empty() {
            self.selected.push(String::from("8.has: clean-up"));
        }

        if !added.is_empty() {
            self.selected.push(String::from("8.has: package (new)"));
        }
    }

    pub fn tags_to_add(&self) -> Vec<String> {
        self.selected.clone()
    }

    pub fn tags_to_remove(&self) -> Vec<String> {
        // The cleanup tag is too vague to automatically remove.
        vec![]
    }
}

pub struct RebuildTagger {
    possible: Vec<String>,
    selected: Vec<String>,
}

impl Default for RebuildTagger {
    fn default() -> RebuildTagger {
        RebuildTagger {
            possible: vec![
                String::from("10.rebuild-darwin: 0"),
                String::from("10.rebuild-darwin: 1"),
                String::from("10.rebuild-darwin: 1-10"),
                String::from("10.rebuild-darwin: 11-100"),
                String::from("10.rebuild-darwin: 101-500"),
                String::from("10.rebuild-darwin: 501+"),
                String::from("10.rebuild-darwin: 501-1000"),
                String::from("10.rebuild-darwin: 1001-2500"),
                String::from("10.rebuild-darwin: 2501-5000"),
                String::from("10.rebuild-darwin: 5001+"),
                String::from("10.rebuild-linux: 0"),
                String::from("10.rebuild-linux: 1"),
                String::from("10.rebuild-linux: 1-10"),
                String::from("10.rebuild-linux: 11-100"),
                String::from("10.rebuild-linux: 101-500"),
                String::from("10.rebuild-linux: 501+"),
                String::from("10.rebuild-linux: 501-1000"),
                String::from("10.rebuild-linux: 1001-2500"),
                String::from("10.rebuild-linux: 2501-5000"),
                String::from("10.rebuild-linux: 5001+"),
            ],
            selected: vec![],
        }
    }
}

impl RebuildTagger {
    pub fn new() -> RebuildTagger {
        Default::default()
    }

    pub fn parse_attrs(&mut self, attrs: Vec<PackageArch>) {
        let mut counter_darwin = 0;
        let mut counter_linux = 0;

        for attr in attrs {
            match attr.architecture.as_ref() {
                "x86_64-darwin" => {
                    counter_darwin += 1;
                }
                "aarch64-darwin" => {}
                "x86_64-linux" => {
                    counter_linux += 1;
                }
                "aarch64-linux" => {}
                "i686-linux" => {}
                arch => {
                    info!("Unknown arch: {:?}", arch);
                }
            }
        }

        self.selected = vec![];
        self.selected.extend(
            RebuildTagger::bucket(counter_darwin)
                .iter()
                .map(|bucket| format!("10.rebuild-darwin: {}", bucket))
                .collect::<Vec<String>>(),
        );

        self.selected.extend(
            RebuildTagger::bucket(counter_linux)
                .iter()
                .map(|bucket| format!("10.rebuild-linux: {}", bucket))
                .collect::<Vec<String>>(),
        );

        for tag in &self.selected {
            if !self.possible.contains(tag) {
                panic!(
                    "Tried to add label {} but it isn't in the possible list!",
                    tag
                );
            }
        }
    }

    pub fn tags_to_add(&self) -> Vec<String> {
        self.selected.clone()
    }

    pub fn tags_to_remove(&self) -> Vec<String> {
        let mut remove = vec![];

        for tag in self.possible.clone().into_iter() {
            if !self.selected.contains(&tag) {
                remove.push(tag);
            }
        }

        remove
    }

    fn bucket(count: u64) -> &'static [&'static str] {
        if count > 5000 {
            &["501+", "5001+"]
        } else if count > 2500 {
            &["501+", "2501-5000"]
        } else if count > 1000 {
            &["501+", "1001-2500"]
        } else if count > 500 {
            &["501+", "501-1000"]
        } else if count > 100 {
            &["101-500"]
        } else if count > 10 {
            &["11-100"]
        } else if count > 1 {
            &["1-10"]
        } else if count > 0 {
            &["1", "1-10"]
        } else {
            &["0"]
        }
    }
}

pub struct MaintainerPrTagger {
    possible: Vec<String>,
    selected: Vec<String>,
}

impl Default for MaintainerPrTagger {
    fn default() -> MaintainerPrTagger {
        let mut t = MaintainerPrTagger {
            possible: vec![String::from("11.by: package-maintainer")],
            selected: vec![],
        };
        t.possible.sort();

        t
    }
}

impl MaintainerPrTagger {
    pub fn new() -> MaintainerPrTagger {
        Default::default()
    }

    pub fn record_maintainer(
        &mut self,
        pr_submitter: &str,
        identified_maintainers: &MaintainersByPackage,
    ) {
        let submitter = Maintainer::from(pr_submitter);

        if identified_maintainers.0.is_empty() {
            // No packages -> not from the maintainer
            return;
        }

        for (_package, maintainers) in identified_maintainers.0.iter() {
            if !maintainers.contains(&submitter) {
                // One of the packages is not maintained by this submitter
                return;
            }
        }

        self.selected
            .push(String::from("11.by: package-maintainer"));
    }

    pub fn tags_to_add(&self) -> Vec<String> {
        self.selected.clone()
    }

    pub fn tags_to_remove(&self) -> Vec<String> {
        // The cleanup tag is too vague to automatically remove.
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct PackageArchSrc {
        linux: usize,
        darwin: usize,
    }

    impl PackageArchSrc {
        pub fn linux(linux: usize) -> PackageArchSrc {
            PackageArchSrc { linux, darwin: 0 }
        }

        pub fn and_darwin(mut self, darwin: usize) -> PackageArchSrc {
            self.darwin = darwin;
            self
        }
    }

    impl From<PackageArchSrc> for Vec<PackageArch> {
        fn from(src: PackageArchSrc) -> Vec<PackageArch> {
            let darwin: Vec<PackageArch> = (0..src.darwin)
                .map(|_| PackageArch {
                    package: String::from("bogus :)"),
                    architecture: String::from("x86_64-darwin"),
                })
                .collect();
            let linux: Vec<PackageArch> = (0..src.linux)
                .map(|_| PackageArch {
                    package: String::from("bogus :)"),
                    architecture: String::from("x86_64-linux"),
                })
                .collect();

            let mut combined = darwin;
            combined.extend(linux);
            combined
        }
    }

    #[test]
    pub fn test_packages_changed() {
        let mut tagger = RebuildTagger::new();
        tagger.parse_attrs(PackageArchSrc::linux(0).and_darwin(0).into());
        assert_eq!(
            tagger.tags_to_add(),
            vec!["10.rebuild-darwin: 0", "10.rebuild-linux: 0",]
        );
        assert_eq!(
            tagger.tags_to_remove(),
            vec![
                "10.rebuild-darwin: 1",
                "10.rebuild-darwin: 1-10",
                "10.rebuild-darwin: 11-100",
                "10.rebuild-darwin: 101-500",
                "10.rebuild-darwin: 501+",
                "10.rebuild-darwin: 501-1000",
                "10.rebuild-darwin: 1001-2500",
                "10.rebuild-darwin: 2501-5000",
                "10.rebuild-darwin: 5001+",
                "10.rebuild-linux: 1",
                "10.rebuild-linux: 1-10",
                "10.rebuild-linux: 11-100",
                "10.rebuild-linux: 101-500",
                "10.rebuild-linux: 501+",
                "10.rebuild-linux: 501-1000",
                "10.rebuild-linux: 1001-2500",
                "10.rebuild-linux: 2501-5000",
                "10.rebuild-linux: 5001+",
            ]
        );

        let mut tagger = RebuildTagger::new();
        tagger.parse_attrs(PackageArchSrc::linux(1).into());

        assert_eq!(
            tagger.tags_to_add(),
            vec![
                "10.rebuild-darwin: 0",
                "10.rebuild-linux: 1",
                "10.rebuild-linux: 1-10",
            ]
        );
        assert_eq!(
            tagger.tags_to_remove(),
            vec![
                "10.rebuild-darwin: 1",
                "10.rebuild-darwin: 1-10",
                "10.rebuild-darwin: 11-100",
                "10.rebuild-darwin: 101-500",
                "10.rebuild-darwin: 501+",
                "10.rebuild-darwin: 501-1000",
                "10.rebuild-darwin: 1001-2500",
                "10.rebuild-darwin: 2501-5000",
                "10.rebuild-darwin: 5001+",
                "10.rebuild-linux: 0",
                "10.rebuild-linux: 11-100",
                "10.rebuild-linux: 101-500",
                "10.rebuild-linux: 501+",
                "10.rebuild-linux: 501-1000",
                "10.rebuild-linux: 1001-2500",
                "10.rebuild-linux: 2501-5000",
                "10.rebuild-linux: 5001+"
            ]
        );

        let mut tagger = RebuildTagger::new();
        tagger.parse_attrs(PackageArchSrc::linux(1).and_darwin(1).into());
        assert_eq!(
            tagger.tags_to_add(),
            vec![
                "10.rebuild-darwin: 1",
                "10.rebuild-darwin: 1-10",
                "10.rebuild-linux: 1",
                "10.rebuild-linux: 1-10",
            ]
        );
        assert_eq!(
            tagger.tags_to_remove(),
            vec![
                "10.rebuild-darwin: 0",
                "10.rebuild-darwin: 11-100",
                "10.rebuild-darwin: 101-500",
                "10.rebuild-darwin: 501+",
                "10.rebuild-darwin: 501-1000",
                "10.rebuild-darwin: 1001-2500",
                "10.rebuild-darwin: 2501-5000",
                "10.rebuild-darwin: 5001+",
                "10.rebuild-linux: 0",
                "10.rebuild-linux: 11-100",
                "10.rebuild-linux: 101-500",
                "10.rebuild-linux: 501+",
                "10.rebuild-linux: 501-1000",
                "10.rebuild-linux: 1001-2500",
                "10.rebuild-linux: 2501-5000",
                "10.rebuild-linux: 5001+"
            ]
        );

        let mut tagger = RebuildTagger::new();
        tagger.parse_attrs(PackageArchSrc::linux(10).and_darwin(10).into());
        assert_eq!(
            tagger.tags_to_add(),
            vec!["10.rebuild-darwin: 1-10", "10.rebuild-linux: 1-10",]
        );
        assert_eq!(
            tagger.tags_to_remove(),
            vec![
                "10.rebuild-darwin: 0",
                "10.rebuild-darwin: 1",
                "10.rebuild-darwin: 11-100",
                "10.rebuild-darwin: 101-500",
                "10.rebuild-darwin: 501+",
                "10.rebuild-darwin: 501-1000",
                "10.rebuild-darwin: 1001-2500",
                "10.rebuild-darwin: 2501-5000",
                "10.rebuild-darwin: 5001+",
                "10.rebuild-linux: 0",
                "10.rebuild-linux: 1",
                "10.rebuild-linux: 11-100",
                "10.rebuild-linux: 101-500",
                "10.rebuild-linux: 501+",
                "10.rebuild-linux: 501-1000",
                "10.rebuild-linux: 1001-2500",
                "10.rebuild-linux: 2501-5000",
                "10.rebuild-linux: 5001+"
            ]
        );

        let mut tagger = RebuildTagger::new();
        tagger.parse_attrs(PackageArchSrc::linux(11).and_darwin(11).into());
        assert_eq!(
            tagger.tags_to_add(),
            vec!["10.rebuild-darwin: 11-100", "10.rebuild-linux: 11-100",]
        );
        assert_eq!(
            tagger.tags_to_remove(),
            vec![
                "10.rebuild-darwin: 0",
                "10.rebuild-darwin: 1",
                "10.rebuild-darwin: 1-10",
                "10.rebuild-darwin: 101-500",
                "10.rebuild-darwin: 501+",
                "10.rebuild-darwin: 501-1000",
                "10.rebuild-darwin: 1001-2500",
                "10.rebuild-darwin: 2501-5000",
                "10.rebuild-darwin: 5001+",
                "10.rebuild-linux: 0",
                "10.rebuild-linux: 1",
                "10.rebuild-linux: 1-10",
                "10.rebuild-linux: 101-500",
                "10.rebuild-linux: 501+",
                "10.rebuild-linux: 501-1000",
                "10.rebuild-linux: 1001-2500",
                "10.rebuild-linux: 2501-5000",
                "10.rebuild-linux: 5001+"
            ]
        );

        let mut tagger = RebuildTagger::new();
        tagger.parse_attrs(PackageArchSrc::linux(100).and_darwin(100).into());
        assert_eq!(
            tagger.tags_to_add(),
            vec!["10.rebuild-darwin: 11-100", "10.rebuild-linux: 11-100",]
        );
        assert_eq!(
            tagger.tags_to_remove(),
            vec![
                "10.rebuild-darwin: 0",
                "10.rebuild-darwin: 1",
                "10.rebuild-darwin: 1-10",
                "10.rebuild-darwin: 101-500",
                "10.rebuild-darwin: 501+",
                "10.rebuild-darwin: 501-1000",
                "10.rebuild-darwin: 1001-2500",
                "10.rebuild-darwin: 2501-5000",
                "10.rebuild-darwin: 5001+",
                "10.rebuild-linux: 0",
                "10.rebuild-linux: 1",
                "10.rebuild-linux: 1-10",
                "10.rebuild-linux: 101-500",
                "10.rebuild-linux: 501+",
                "10.rebuild-linux: 501-1000",
                "10.rebuild-linux: 1001-2500",
                "10.rebuild-linux: 2501-5000",
                "10.rebuild-linux: 5001+"
            ]
        );

        let mut tagger = RebuildTagger::new();
        tagger.parse_attrs(PackageArchSrc::linux(101).and_darwin(101).into());
        assert_eq!(
            tagger.tags_to_add(),
            vec!["10.rebuild-darwin: 101-500", "10.rebuild-linux: 101-500",]
        );
        assert_eq!(
            tagger.tags_to_remove(),
            vec![
                "10.rebuild-darwin: 0",
                "10.rebuild-darwin: 1",
                "10.rebuild-darwin: 1-10",
                "10.rebuild-darwin: 11-100",
                "10.rebuild-darwin: 501+",
                "10.rebuild-darwin: 501-1000",
                "10.rebuild-darwin: 1001-2500",
                "10.rebuild-darwin: 2501-5000",
                "10.rebuild-darwin: 5001+",
                "10.rebuild-linux: 0",
                "10.rebuild-linux: 1",
                "10.rebuild-linux: 1-10",
                "10.rebuild-linux: 11-100",
                "10.rebuild-linux: 501+",
                "10.rebuild-linux: 501-1000",
                "10.rebuild-linux: 1001-2500",
                "10.rebuild-linux: 2501-5000",
                "10.rebuild-linux: 5001+"
            ]
        );

        let mut tagger = RebuildTagger::new();
        tagger.parse_attrs(PackageArchSrc::linux(500).and_darwin(500).into());
        assert_eq!(
            tagger.tags_to_add(),
            vec!["10.rebuild-darwin: 101-500", "10.rebuild-linux: 101-500",]
        );
        assert_eq!(
            tagger.tags_to_remove(),
            vec![
                "10.rebuild-darwin: 0",
                "10.rebuild-darwin: 1",
                "10.rebuild-darwin: 1-10",
                "10.rebuild-darwin: 11-100",
                "10.rebuild-darwin: 501+",
                "10.rebuild-darwin: 501-1000",
                "10.rebuild-darwin: 1001-2500",
                "10.rebuild-darwin: 2501-5000",
                "10.rebuild-darwin: 5001+",
                "10.rebuild-linux: 0",
                "10.rebuild-linux: 1",
                "10.rebuild-linux: 1-10",
                "10.rebuild-linux: 11-100",
                "10.rebuild-linux: 501+",
                "10.rebuild-linux: 501-1000",
                "10.rebuild-linux: 1001-2500",
                "10.rebuild-linux: 2501-5000",
                "10.rebuild-linux: 5001+"
            ]
        );

        let mut tagger = RebuildTagger::new();
        tagger.parse_attrs(PackageArchSrc::linux(501).and_darwin(501).into());
        assert_eq!(
            tagger.tags_to_add(),
            vec![
                "10.rebuild-darwin: 501+",
                "10.rebuild-darwin: 501-1000",
                "10.rebuild-linux: 501+",
                "10.rebuild-linux: 501-1000",
            ]
        );
        assert_eq!(
            tagger.tags_to_remove(),
            vec![
                "10.rebuild-darwin: 0",
                "10.rebuild-darwin: 1",
                "10.rebuild-darwin: 1-10",
                "10.rebuild-darwin: 11-100",
                "10.rebuild-darwin: 101-500",
                "10.rebuild-darwin: 1001-2500",
                "10.rebuild-darwin: 2501-5000",
                "10.rebuild-darwin: 5001+",
                "10.rebuild-linux: 0",
                "10.rebuild-linux: 1",
                "10.rebuild-linux: 1-10",
                "10.rebuild-linux: 11-100",
                "10.rebuild-linux: 101-500",
                "10.rebuild-linux: 1001-2500",
                "10.rebuild-linux: 2501-5000",
                "10.rebuild-linux: 5001+"
            ]
        );

        let mut tagger = RebuildTagger::new();
        tagger.parse_attrs(PackageArchSrc::linux(1000).and_darwin(1000).into());
        assert_eq!(
            tagger.tags_to_add(),
            vec![
                "10.rebuild-darwin: 501+",
                "10.rebuild-darwin: 501-1000",
                "10.rebuild-linux: 501+",
                "10.rebuild-linux: 501-1000",
            ]
        );
        assert_eq!(
            tagger.tags_to_remove(),
            vec![
                "10.rebuild-darwin: 0",
                "10.rebuild-darwin: 1",
                "10.rebuild-darwin: 1-10",
                "10.rebuild-darwin: 11-100",
                "10.rebuild-darwin: 101-500",
                "10.rebuild-darwin: 1001-2500",
                "10.rebuild-darwin: 2501-5000",
                "10.rebuild-darwin: 5001+",
                "10.rebuild-linux: 0",
                "10.rebuild-linux: 1",
                "10.rebuild-linux: 1-10",
                "10.rebuild-linux: 11-100",
                "10.rebuild-linux: 101-500",
                "10.rebuild-linux: 1001-2500",
                "10.rebuild-linux: 2501-5000",
                "10.rebuild-linux: 5001+"
            ]
        );

        let mut tagger = RebuildTagger::new();
        tagger.parse_attrs(PackageArchSrc::linux(1001).and_darwin(1001).into());
        assert_eq!(
            tagger.tags_to_add(),
            vec![
                "10.rebuild-darwin: 501+",
                "10.rebuild-darwin: 1001-2500",
                "10.rebuild-linux: 501+",
                "10.rebuild-linux: 1001-2500"
            ]
        );
        assert_eq!(
            tagger.tags_to_remove(),
            vec![
                "10.rebuild-darwin: 0",
                "10.rebuild-darwin: 1",
                "10.rebuild-darwin: 1-10",
                "10.rebuild-darwin: 11-100",
                "10.rebuild-darwin: 101-500",
                "10.rebuild-darwin: 501-1000",
                "10.rebuild-darwin: 2501-5000",
                "10.rebuild-darwin: 5001+",
                "10.rebuild-linux: 0",
                "10.rebuild-linux: 1",
                "10.rebuild-linux: 1-10",
                "10.rebuild-linux: 11-100",
                "10.rebuild-linux: 101-500",
                "10.rebuild-linux: 501-1000",
                "10.rebuild-linux: 2501-5000",
                "10.rebuild-linux: 5001+"
            ]
        );

        let mut tagger = RebuildTagger::new();
        tagger.parse_attrs(PackageArchSrc::linux(2500).and_darwin(2500).into());
        assert_eq!(
            tagger.tags_to_add(),
            vec![
                "10.rebuild-darwin: 501+",
                "10.rebuild-darwin: 1001-2500",
                "10.rebuild-linux: 501+",
                "10.rebuild-linux: 1001-2500"
            ]
        );
        assert_eq!(
            tagger.tags_to_remove(),
            vec![
                "10.rebuild-darwin: 0",
                "10.rebuild-darwin: 1",
                "10.rebuild-darwin: 1-10",
                "10.rebuild-darwin: 11-100",
                "10.rebuild-darwin: 101-500",
                "10.rebuild-darwin: 501-1000",
                "10.rebuild-darwin: 2501-5000",
                "10.rebuild-darwin: 5001+",
                "10.rebuild-linux: 0",
                "10.rebuild-linux: 1",
                "10.rebuild-linux: 1-10",
                "10.rebuild-linux: 11-100",
                "10.rebuild-linux: 101-500",
                "10.rebuild-linux: 501-1000",
                "10.rebuild-linux: 2501-5000",
                "10.rebuild-linux: 5001+"
            ]
        );

        let mut tagger = RebuildTagger::new();
        tagger.parse_attrs(PackageArchSrc::linux(2501).and_darwin(2501).into());
        assert_eq!(
            tagger.tags_to_add(),
            vec![
                "10.rebuild-darwin: 501+",
                "10.rebuild-darwin: 2501-5000",
                "10.rebuild-linux: 501+",
                "10.rebuild-linux: 2501-5000"
            ]
        );
        assert_eq!(
            tagger.tags_to_remove(),
            vec![
                "10.rebuild-darwin: 0",
                "10.rebuild-darwin: 1",
                "10.rebuild-darwin: 1-10",
                "10.rebuild-darwin: 11-100",
                "10.rebuild-darwin: 101-500",
                "10.rebuild-darwin: 501-1000",
                "10.rebuild-darwin: 1001-2500",
                "10.rebuild-darwin: 5001+",
                "10.rebuild-linux: 0",
                "10.rebuild-linux: 1",
                "10.rebuild-linux: 1-10",
                "10.rebuild-linux: 11-100",
                "10.rebuild-linux: 101-500",
                "10.rebuild-linux: 501-1000",
                "10.rebuild-linux: 1001-2500",
                "10.rebuild-linux: 5001+"
            ]
        );

        let mut tagger = RebuildTagger::new();
        tagger.parse_attrs(PackageArchSrc::linux(5000).and_darwin(5000).into());
        assert_eq!(
            tagger.tags_to_add(),
            vec![
                "10.rebuild-darwin: 501+",
                "10.rebuild-darwin: 2501-5000",
                "10.rebuild-linux: 501+",
                "10.rebuild-linux: 2501-5000"
            ]
        );
        assert_eq!(
            tagger.tags_to_remove(),
            vec![
                "10.rebuild-darwin: 0",
                "10.rebuild-darwin: 1",
                "10.rebuild-darwin: 1-10",
                "10.rebuild-darwin: 11-100",
                "10.rebuild-darwin: 101-500",
                "10.rebuild-darwin: 501-1000",
                "10.rebuild-darwin: 1001-2500",
                "10.rebuild-darwin: 5001+",
                "10.rebuild-linux: 0",
                "10.rebuild-linux: 1",
                "10.rebuild-linux: 1-10",
                "10.rebuild-linux: 11-100",
                "10.rebuild-linux: 101-500",
                "10.rebuild-linux: 501-1000",
                "10.rebuild-linux: 1001-2500",
                "10.rebuild-linux: 5001+"
            ]
        );

        let mut tagger = RebuildTagger::new();
        tagger.parse_attrs(PackageArchSrc::linux(5001).and_darwin(5001).into());
        assert_eq!(
            tagger.tags_to_add(),
            vec![
                "10.rebuild-darwin: 501+",
                "10.rebuild-darwin: 5001+",
                "10.rebuild-linux: 501+",
                "10.rebuild-linux: 5001+"
            ]
        );
        assert_eq!(
            tagger.tags_to_remove(),
            vec![
                "10.rebuild-darwin: 0",
                "10.rebuild-darwin: 1",
                "10.rebuild-darwin: 1-10",
                "10.rebuild-darwin: 11-100",
                "10.rebuild-darwin: 101-500",
                "10.rebuild-darwin: 501-1000",
                "10.rebuild-darwin: 1001-2500",
                "10.rebuild-darwin: 2501-5000",
                "10.rebuild-linux: 0",
                "10.rebuild-linux: 1",
                "10.rebuild-linux: 1-10",
                "10.rebuild-linux: 11-100",
                "10.rebuild-linux: 101-500",
                "10.rebuild-linux: 501-1000",
                "10.rebuild-linux: 1001-2500",
                "10.rebuild-linux: 2501-5000",
            ]
        );
    }
}
