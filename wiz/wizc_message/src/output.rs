use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::path::Path;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Message {
    kind: MessageKind,
}

impl Message {
    pub fn new(kind: MessageKind) -> Self {
        Self { kind }
    }

    fn info(message: &str) -> Self {
        Self::new(MessageKind::Info(message.to_string()))
    }

    fn warn(message: &str) -> Self {
        Self::new(MessageKind::Warn(message.to_string()))
    }

    fn error(message: &str) -> Self {
        Self::new(MessageKind::Error(message.to_string()))
    }

    fn output<P: AsRef<Path>>(&self, path: P) -> Self {
        Self::new(MessageKind::Output(path.as_ref().display().to_string()))
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
