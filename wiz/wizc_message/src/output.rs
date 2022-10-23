use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Message {
    kind: MessageKind,
}

impl Message {
    pub fn new(kind: MessageKind) -> Self {
        Self { kind }
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
