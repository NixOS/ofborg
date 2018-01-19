use cmdlog::Logger;

pub struct NullLogger {
}

impl NullLogger {
    pub fn new() -> NullLogger {
        NullLogger{}
    }
}

impl Logger for NullLogger {
    fn build_output(&mut self, _: &str) {}
}
