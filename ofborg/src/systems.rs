#[derive(Clone, Debug)]
pub enum System {
    X8664Linux,
    Aarch64Linux,
    X8664Darwin,
}

impl System {
    pub fn to_string(&self) -> String {
        match self {
            System::X8664Linux => String::from("x86_64-linux"),
            System::Aarch64Linux => String::from("aarch64-linux"),
            System::X8664Darwin => String::from("x86_64-darwin"),
        }
    }

    pub fn as_build_destination(&self) -> (Option<String>, Option<String>) {
        (None, Some(format!("build-inputs-{}", self.to_string())))
    }
}
