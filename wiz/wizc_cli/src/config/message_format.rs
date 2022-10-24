#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum MessageFormat {
    Normal,
    Json,
}

impl MessageFormat {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Normal => "normal",
            Self::Json => "json"
        }
    }
}

impl Default for MessageFormat {
    fn default() -> Self {
        Self::Normal
    }
}

impl From<&str> for MessageFormat {
    fn from(value: &str) -> Self {
        match value {
            "json" => Self::Json,
            _ => Self::Normal,
        }
    }
}
