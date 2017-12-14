use ofborg::tasks;

pub struct StdenvTagger {
    possible: Vec<String>,
    selected: Vec<String>,
}

impl StdenvTagger {
    pub fn new() -> StdenvTagger {
        let mut t = StdenvTagger {
            possible: vec![
                String::from("10.rebuild-linux-stdenv"),
                String::from("10.rebuild-darwin-stdenv"),
            ],
            selected: vec![],
        };
        t.possible.sort();

        return t;
    }

    pub fn changed(&mut self, systems: Vec<tasks::massrebuilder::System>) {
        for system in systems {
            match system {
                tasks::massrebuilder::System::X8664Darwin => {
                    self.selected.push(String::from("10.rebuild-darwin-stdenv"));
                }
                tasks::massrebuilder::System::X8664Linux => {
                    self.selected.push(String::from("10.rebuild-linux-stdenv"));
                }
            }
        }

        for tag in &self.selected {
            if !self.possible.contains(&tag) {
                panic!("Tried to add label {} but it isn't in the possible list!", tag);
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

        return remove;
    }
}


pub struct RebuildTagger {
    possible: Vec<String>,
    selected: Vec<String>,
}

impl RebuildTagger {
    pub fn new() -> RebuildTagger {
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

        return t;
    }

    pub fn parse_attrs(&mut self, attrs: Vec<String>) {
        let mut counter_darwin = 0;
        let mut counter_linux = 0;

        for attr in attrs {
            match attr.rsplit(".").next() {
                Some("x86_64-darwin") => { counter_darwin += 1; }
                Some("x86_64-linux") => { counter_linux += 1; }
                Some("aarch64-linux") => { }
                Some("i686-linux") => { }
                Some(arch) => { info!("Unknown arch: {:?}", arch); }
                None => { info!("Cannot grok attr: {:?}", attr); }
            }
        }

        self.selected = vec![
            String::from(format!("10.rebuild-linux: {}", self.bucket(counter_linux))),
            String::from(format!("10.rebuild-darwin: {}", self.bucket(counter_darwin))),
        ];

        for tag in &self.selected {
            if !self.possible.contains(&tag) {
                panic!("Tried to add label {} but it isn't in the possible list!", tag);
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

        return remove;
    }

    fn bucket(&self, count: u64) -> &str{
        if count > 500 {
            return "501+";
        } else if count > 100 {
            return "101-500";
        } else if count > 10 {
            return "11-100";
        } else if count > 0 {
            return "1-10";
        } else {
            return "0";
        }

    }
}
