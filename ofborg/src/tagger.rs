use ofborg::outpathdiff::PackageArch;
use ofborg::tasks;
use std::collections::HashMap;

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
            if !self.possible.contains(&tag) {
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
            let pos = remove.binary_search(&tag).unwrap();
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
        let mut t = RebuildTagger {
            possible: vec![
                String::from("10.rebuild-linux: 501+"),
                String::from("10.rebuild-linux: 101-500"),
                String::from("10.rebuild-linux: 11-100"),
                String::from("10.rebuild-linux: 1-10"),
                String::from("10.rebuild-linux: 0"),
                String::from("10.rebuild-darwin: 501+"),
                String::from("10.rebuild-darwin: 101-500"),
                String::from("10.rebuild-darwin: 11-100"),
                String::from("10.rebuild-darwin: 1-10"),
                String::from("10.rebuild-darwin: 0"),
            ],
            selected: vec![],
        };
        t.possible.sort();

        t
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

        self.selected = vec![
            format!("10.rebuild-linux: {}", self.bucket(counter_linux)),
            format!("10.rebuild-darwin: {}", self.bucket(counter_darwin)),
        ];

        for tag in &self.selected {
            if !self.possible.contains(&tag) {
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
            let pos = remove.binary_search(&tag).unwrap();
            remove.remove(pos);
        }

        remove
    }

    fn bucket(&self, count: u64) -> &str {
        if count > 500 {
            "501+"
        } else if count > 100 {
            "101-500"
        } else if count > 10 {
            "11-100"
        } else if count > 0 {
            "1-10"
        } else {
            "0"
        }
    }
}

pub struct PathsTagger {
    possible: HashMap<String, Vec<String>>,
    selected: Vec<String>,
}

impl PathsTagger {
    pub fn new(tags_and_criteria: HashMap<String, Vec<String>>) -> PathsTagger {
        PathsTagger {
            possible: tags_and_criteria,
            selected: vec![],
        }
    }

    pub fn path_changed(&mut self, path: &str) {
        let mut tags_to_add: Vec<String> = self
            .possible
            .iter()
            .filter(|&(ref tag, ref _paths)| !self.selected.contains(&tag))
            .filter(|&(ref _tag, ref paths)| paths.iter().any(|tp| path.contains(tp)))
            .map(|(tag, _paths)| tag.clone())
            .collect();
        self.selected.append(&mut tags_to_add);
        self.selected.sort();
    }

    pub fn tags_to_add(&self) -> Vec<String> {
        self.selected.clone()
    }

    pub fn tags_to_remove(&self) -> Vec<String> {
        let mut remove: Vec<String> = self.possible.keys().map(|k| k.to_owned()).collect();
        remove.sort();
        for tag in &self.selected {
            let pos = remove.binary_search(&tag).unwrap();
            remove.remove(pos);
        }

        remove
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_files_changed_list() {
        let mut criteria: HashMap<String, Vec<String>> = HashMap::new();
        criteria.insert(
            "topic: python".to_owned(),
            vec![
                "pkgs/top-level/python-packages.nix".to_owned(),
                "bogus".to_owned(),
            ],
        );
        criteria.insert(
            "topic: ruby".to_owned(),
            vec![
                "pkgs/development/interpreters/ruby".to_owned(),
                "bogus".to_owned(),
            ],
        );

        {
            let mut tagger = PathsTagger::new(criteria.clone());
            tagger.path_changed("default.nix");
            assert_eq!(tagger.tags_to_add().len(), 0);
            assert_eq!(
                tagger.tags_to_remove(),
                vec!["topic: python".to_owned(), "topic: ruby".to_owned()]
            );

            tagger.path_changed("pkgs/development/interpreters/ruby/default.nix");
            assert_eq!(tagger.tags_to_add(), vec!["topic: ruby".to_owned()]);
            assert_eq!(tagger.tags_to_remove(), vec!["topic: python".to_owned()]);

            tagger.path_changed("pkgs/development/interpreters/ruby/foobar.nix");
            assert_eq!(tagger.tags_to_add(), vec!["topic: ruby".to_owned()]);
            assert_eq!(tagger.tags_to_remove(), vec!["topic: python".to_owned()]);

            tagger.path_changed("pkgs/top-level/python-packages.nix");
            assert_eq!(
                tagger.tags_to_add(),
                vec!["topic: python".to_owned(), "topic: ruby".to_owned()]
            );
        }

        {
            let mut tagger = PathsTagger::new(criteria.clone());
            tagger.path_changed("bogus");
            assert_eq!(
                tagger.tags_to_add(),
                vec!["topic: python".to_owned(), "topic: ruby".to_owned()]
            );
        }
    }
}
