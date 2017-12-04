
use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use toml;

#[derive(Serialize, Deserialize, Debug)]
pub struct Factoids {
    pub factoids: HashMap<String, String>
}

impl Factoids {
    pub fn load(src: &Path) -> Factoids {
        let mut file = File::open(src).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        return toml::from_str(&contents).unwrap();
    }
}
