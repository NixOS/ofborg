use std::io::BufReader;
use std::io::BufRead;
use std::io::Write;
use std::io::Seek;
use std::io::SeekFrom;
use std::fs::File;

pub fn write_to_line(rw: &mut File, line: usize, data: &str) {

    rw.seek(SeekFrom::Start(0)).unwrap();

    let reader = BufReader::new(rw.try_clone().unwrap());
    let mut lines: Vec<String> = reader
        .lines()
        .map(|line| match line {
            Ok(s) => s,
            Err(e) => format!("UTF-8 Decode err: {:?}", e),
        })
        .collect();
    while lines.len() <= line {
        lines.push("".to_owned());
    }

    lines.remove(line);
    lines.insert(line, data.to_owned());

    let writeout = lines.join("\n");

    rw.set_len(0).unwrap();
    rw.seek(SeekFrom::Start(0)).unwrap();

    let bytes = writeout.as_bytes();
    rw.write_all(bytes).unwrap();
    rw.write("\n".as_bytes()).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;
    use std::path::Path;
    use std::path::PathBuf;
    use std::fs::File;
    use std::io::Read;
    use std::fs::OpenOptions;

    fn tpath(component: &str) -> PathBuf {
        return Path::new(env!("CARGO_MANIFEST_DIR")).join(component);
    }

    fn scratch_file(name: &str) -> PathBuf {
        tpath(&format!("./scratch-write-to-line-{}", name))
    }

    fn cleanup_scratch(name: &str) {
        Command::new("rm")
            .arg("-f")
            .arg(&scratch_file(name))
            .status()
            .expect("cleanup of scratch_dir should work");
    }

    fn testfile(name: &str) -> File {
        cleanup_scratch(&name);
        OpenOptions::new()
            .read(true)
            .write(true)
            .truncate(true)
            .create(true)
            .open(scratch_file(&name))
            .expect("failed to open scratch file")
    }

    fn assert_file_content<T>(f: &mut T, value: &str)
    where
        T: Read + Seek,
    {
        let mut mystr: String = String::new();
        f.seek(SeekFrom::Start(0)).unwrap();
        f.read_to_string(&mut mystr).unwrap();
        assert_eq!(mystr, value);
    }

    #[test]
    fn test_writer_line_ordered() {
        let mut f = testfile("ordered");

        assert_file_content(&mut f, "");
        write_to_line(&mut f, 0, "hello");
        assert_file_content(&mut f, "hello\n");
        write_to_line(&mut f, 1, "world");
        assert_file_content(&mut f, "hello\nworld\n");
        write_to_line(&mut f, 2, ":)");
        assert_file_content(&mut f, "hello\nworld\n:)\n");
    }

    #[test]
    fn test_writer_line_unordered() {
        let mut f = testfile("unordered");

        assert_file_content(&mut f, "");
        write_to_line(&mut f, 2, ":)");
        assert_file_content(&mut f, "\n\n:)\n");

        write_to_line(&mut f, 1, "world");
        assert_file_content(&mut f, "\nworld\n:)\n");

        write_to_line(&mut f, 0, "hello");
        assert_file_content(&mut f, "hello\nworld\n:)\n");
    }


    #[test]
    fn test_writer_line_unordered_long() {
        let mut f = testfile("unordered-long");

        assert_file_content(&mut f, "");
        write_to_line(
            &mut f,
            2,
            "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
        );
        assert_file_content(
            &mut f,
            "\n\nAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\n",
        );

        write_to_line(
            &mut f,
            1,
            "BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB",
        );
        assert_file_content(
            &mut f,
            "\nBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB\nAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\n",
        );

        write_to_line(
            &mut f,
            0,
            "CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC",
        );
        assert_file_content(
            &mut f,
            "CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC\nBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB\nAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\n",
        );
    }


    #[test]
    fn test_writer_line_unordered_longish() {
        let mut f = testfile("unordered-longish");

        assert_file_content(&mut f, "");
        write_to_line(&mut f, 2, "hello");
        assert_file_content(&mut f, "\n\nhello\n");

        write_to_line(&mut f, 1, "mynameis");
        assert_file_content(&mut f, "\nmynameis\nhello\n");

        write_to_line(&mut f, 0, "graham");
        assert_file_content(&mut f, "graham\nmynameis\nhello\n");
    }

    #[test]
    fn test_writer_line_middle() {
        let mut f = testfile("middle");

        assert_file_content(&mut f, "");
        write_to_line(&mut f, 5, "hello");
        assert_file_content(&mut f, "\n\n\n\n\nhello\n");
    }
}
