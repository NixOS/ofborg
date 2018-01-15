use std::collections::VecDeque;
use cmdlog::Logger;

pub struct LastNLogger {
    buffer: VecDeque<String>,
    max: usize,

}

impl LastNLogger {
    pub fn new(n: usize) -> LastNLogger {
        LastNLogger {
            max: n,
            buffer: VecDeque::with_capacity(n)
        }
    }

    pub fn lines(self) -> Vec<String> {
        return self.buffer.into_iter().collect::<Vec<String>>();
    }
}

impl Logger for LastNLogger {
    fn build_output(&mut self, line: &str) {
        if self.buffer.len() >= 10 {
            self.buffer.pop_front();
        }

        self.buffer.push_back(line.to_owned());
    }
}
