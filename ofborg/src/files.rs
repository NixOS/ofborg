use std::fs::File;
use std::io::Read;

pub fn file_to_str(f: &mut File) -> String {
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).expect("Reading eval output");
    String::from(String::from_utf8_lossy(&buffer))
}
