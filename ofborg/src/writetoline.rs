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
    use std::path::Path;
    use std::fs::File;
    use std::io::Read;
    use std::fs::OpenOptions;
    use ofborg::test_scratch::TestScratch;

    fn testfile(path: &Path) -> File {
        OpenOptions::new()
            .read(true)
            .write(true)
            .truncate(true)
            .create(true)
            .open(path)
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
        let p = TestScratch::new_file("writetoline-ordered");
        let mut f = testfile(&p.path());

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
        let p = TestScratch::new_file("writetoline-unordered");
        let mut f = testfile(&p.path());

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
        let p = TestScratch::new_file("writetoline-unordered-long");
        let mut f = testfile(&p.path());

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
        let p = TestScratch::new_file("writetoline-unordered-longish");
        let mut f = testfile(&p.path());

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
        let p = TestScratch::new_file("writetoline-middle");
        let mut f = testfile(&p.path());

        assert_file_content(&mut f, "");
        write_to_line(&mut f, 5, "hello");
        assert_file_content(&mut f, "\n\n\n\n\nhello\n");
    }
}
