#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildType {
    Binary,
    Library,
    Test,
}

impl BuildType {
    pub fn all_str() -> &'static [&'static str] {
        &["bin", "lib", "test"]
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            BuildType::Binary => "bin",
            BuildType::Library => "lib",
            BuildType::Test => "test",
        }
    }
}

impl From<&str> for BuildType {
    fn from(s: &str) -> Self {
        match s {
            "bin" => BuildType::Binary,
            "lib" => BuildType::Library,
            "test" => BuildType::Test,
            _ => panic!("Unknown build type: {}", s),
        }
    }
}
