use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};
use std::path::Path;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Message {
    pub kind: MessageKind,
}

impl Message {
    pub fn new(kind: MessageKind) -> Self {
        Self { kind }
    }

    pub fn info(message: &str) -> Self {
        Self::new(MessageKind::Info(message.to_string()))
    }

    pub fn warn(message: &str) -> Self {
        Self::new(MessageKind::Warn(message.to_string()))
    }

    pub fn error(message: &str) -> Self {
        Self::new(MessageKind::Error(message.to_string()))
    }

    pub fn output<P: AsRef<Path>>(path: P) -> Self {
        Self::new(MessageKind::Output(path.as_ref().display().to_string()))
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.kind.fmt(f)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum MessageKind {
    Output(String),
    Info(String),
    Warn(String),
    Error(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn message_kind_output_serialize_deserialize() {
        let message = Message::new(MessageKind::Output(String::from(
            "/home/user/test/target/output.json",
        )));
        let str = serde_json::to_string(&message).unwrap();
        let deserialized = serde_json::from_str(&str).unwrap();
        assert_eq!(message, deserialized)
    }

    #[test]
    fn message_kind_info_serialize_deserialize() {
        let message = Message::new(MessageKind::Info(String::from("Error message")));
        let str = serde_json::to_string(&message).unwrap();
        let deserialized = serde_json::from_str(&str).unwrap();
        assert_eq!(message, deserialized)
    }

    #[test]
    fn message_kind_warn_serialize_deserialize() {
        let message = Message::new(MessageKind::Warn(String::from("Error message")));
        let str = serde_json::to_string(&message).unwrap();
        let deserialized = serde_json::from_str(&str).unwrap();
        assert_eq!(message, deserialized)
    }

    #[test]
    fn message_kind_error_serialize_deserialize() {
        let message = Message::new(MessageKind::Error(String::from("Error message")));
        let str = serde_json::to_string(&message).unwrap();
        let deserialized = serde_json::from_str(&str).unwrap();
        assert_eq!(message, deserialized)
    }
}
