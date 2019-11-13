use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;

pub struct LineWriter {
    file: File,
    buffer: Vec<String>,
    last_line: usize,
}

impl LineWriter {
    pub fn new(mut rw: File) -> LineWriter {
        let buf = LineWriter::load_buffer(&mut rw);
        let len = buf.len();

        LineWriter {
            file: rw,
            buffer: buf,
            last_line: len,
        }
    }

    fn load_buffer(file: &mut File) -> Vec<String> {
        file.seek(SeekFrom::Start(0)).unwrap();

        let reader = BufReader::new(file.try_clone().unwrap());
        reader
            .lines()
            .map(|line| match line {
                Ok(s) => s,
                Err(e) => format!("UTF-8 Decode err: {:?}", e),
            })
            .collect()
    }

    pub fn write_to_line(&mut self, line: usize, data: &str) {
        let original_len = self.buffer.len();
        while self.buffer.len() <= line {
            self.buffer.push("".to_owned());
        }

        self.buffer.remove(line);
        self.buffer.insert(line, data.to_owned());

        if self.last_line > line {
            // println!("taking the rewrite option");
            // We're inserting in to the middle of a file, so just
            // write the entire buffer again
            self.file.set_len(0).unwrap();
            self.file.seek(SeekFrom::Start(0)).unwrap();
            self.file
                .write_all(self.buffer.join("\n").as_bytes())
                .unwrap();
            self.file.write_all(b"\n").unwrap();
        } else {
            // println!("taking the append option");
            // println!("Writing {:?} to line {}", data, line);

            let buffer_start = original_len;
            let buffer_end = line + 1;
            let to_write = self.buffer[buffer_start..buffer_end].join("\n");
            // println!("Full buffer: {:?}", self.buffer);
            // println!("buffer[{}..{}] = {:?}", buffer_start, buffer_end, to_write);
            // Inclusive range syntax (ie: ...) is experimental, so
            // to include the final newline in to the written buffer
            // we have to use one more than the range we want for the
            // end
            // println!("selected buffer: {:?}", to_write);
            self.file.write_all(to_write.as_bytes()).unwrap();
            self.file.write_all(b"\n").unwrap();
        }

        self.last_line = line;
    }

    pub fn inner(self) -> File {
        self.file
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_scratch::TestScratch;
    use std::fs::File;
    use std::fs::OpenOptions;
    use std::io::Read;
    use std::path::Path;
    use std::time::Instant;

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

        let mut writer = LineWriter::new(f);
        writer.write_to_line(0, "hello");
        f = writer.inner();

        assert_file_content(&mut f, "hello\n");

        let mut writer = LineWriter::new(f);
        writer.write_to_line(1, "world");
        f = writer.inner();

        assert_file_content(&mut f, "hello\nworld\n");

        let mut writer = LineWriter::new(f);
        writer.write_to_line(2, ":)");
        f = writer.inner();

        assert_file_content(&mut f, "hello\nworld\n:)\n");
    }

    #[test]
    fn test_writer_line_unordered() {
        let p = TestScratch::new_file("writetoline-unordered");
        let mut f = testfile(&p.path());

        assert_file_content(&mut f, "");

        {
            let mut writer = LineWriter::new(f);
            writer.write_to_line(2, ":)");
            f = writer.inner();
        }

        assert_file_content(&mut f, "\n\n:)\n");

        {
            let mut writer = LineWriter::new(f);
            writer.write_to_line(1, "world");
            f = writer.inner();
        }

        assert_file_content(&mut f, "\nworld\n:)\n");

        {
            let mut writer = LineWriter::new(f);
            writer.write_to_line(0, "hello");
            f = writer.inner();
        }

        assert_file_content(&mut f, "hello\nworld\n:)\n");
    }

    #[test]
    fn test_writer_line_unordered_long() {
        let p = TestScratch::new_file("writetoline-unordered-long");
        let mut f = testfile(&p.path());

        assert_file_content(&mut f, "");

        {
            let mut writer = LineWriter::new(f);
            writer.write_to_line(
                2,
                "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
            );
            f = writer.inner();
        }
        assert_file_content(
            &mut f,
            "\n\nAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\n",
        );

        {
            let mut writer = LineWriter::new(f);
            writer.write_to_line(
                1,
                "BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB",
            );
            f = writer.inner();
        }
        assert_file_content(
            &mut f,
            "\nBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB\nAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\n",
        );

        {
            let mut writer = LineWriter::new(f);
            writer.write_to_line(
                0,
                "CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC",
            );
            f = writer.inner();
        }
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

        {
            let mut writer = LineWriter::new(f);
            writer.write_to_line(2, "hello");
            f = writer.inner();
        }
        assert_file_content(&mut f, "\n\nhello\n");

        {
            let mut writer = LineWriter::new(f);
            writer.write_to_line(1, "mynameis");
            f = writer.inner();
        }
        assert_file_content(&mut f, "\nmynameis\nhello\n");

        {
            let mut writer = LineWriter::new(f);
            writer.write_to_line(0, "graham");
            f = writer.inner();
        }
        assert_file_content(&mut f, "graham\nmynameis\nhello\n");
    }

    #[test]
    fn test_writer_line_ordered_result() {
        let p = TestScratch::new_file("writetoline-ordered-result");
        let mut f = testfile(&p.path());

        let mut writer = LineWriter::new(f);
        writer.write_to_line(0, "hello");
        writer.write_to_line(1, "world");
        writer.write_to_line(2, ":)");
        f = writer.inner();

        assert_file_content(&mut f, "hello\nworld\n:)\n");
    }

    #[test]
    fn test_writer_line_unordered_result() {
        let p = TestScratch::new_file("writetoline-unordered-result");
        let mut f = testfile(&p.path());

        let mut writer = LineWriter::new(f);
        writer.write_to_line(2, ":)");
        writer.write_to_line(1, "world");
        writer.write_to_line(0, "hello");
        f = writer.inner();

        assert_file_content(&mut f, "hello\nworld\n:)\n");
    }

    #[test]
    fn test_writer_line_unordered_long_result() {
        let p = TestScratch::new_file("writetoline-unordered-long-result");
        let mut f = testfile(&p.path());

        let mut writer = LineWriter::new(f);
        writer.write_to_line(
            2,
            "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
        );
        writer.write_to_line(
            1,
            "BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB",
        );
        writer.write_to_line(
            0,
            "CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC",
        );
        f = writer.inner();

        assert_file_content(
            &mut f,
            "CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC\nBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB\nAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\n",
        );
    }

    #[test]
    fn test_writer_line_unordered_longish_result() {
        let p = TestScratch::new_file("writetoline-unordered-longish-result");
        let mut f = testfile(&p.path());

        let mut writer = LineWriter::new(f);
        writer.write_to_line(2, "hello");
        writer.write_to_line(1, "mynameis");
        writer.write_to_line(0, "graham");
        f = writer.inner();

        assert_file_content(&mut f, "graham\nmynameis\nhello\n");
    }

    #[test]
    fn test_writer_line_middle() {
        let p = TestScratch::new_file("writetoline-middle");
        let mut f = testfile(&p.path());

        assert_file_content(&mut f, "");

        {
            let mut writer = LineWriter::new(f);
            writer.write_to_line(5, "hello");
            f = writer.inner();
        }
        assert_file_content(&mut f, "\n\n\n\n\nhello\n");
    }

    #[test]
    fn bench_lots_of_ordered_lines() {
        let p = TestScratch::new_file("bench-ordered-lines");
        let f = testfile(&p.path());
        let mut writer = LineWriter::new(f);

        let timer = Instant::now();

        for i in 0..3000 {
            writer.write_to_line(i, "This is my line!");
        }

        println!("ordered took: {:?}", timer.elapsed());
    }

    #[test]
    fn bench_lots_of_reversed_lines() {
        let p = TestScratch::new_file("bench-reversed-lines");
        let f = testfile(&p.path());
        let mut writer = LineWriter::new(f);

        let timer = Instant::now();

        for i in (0..3000).rev() {
            writer.write_to_line(i, "This is my line!");
        }

        println!("reversed took: {:?}", timer.elapsed());
    }
}
