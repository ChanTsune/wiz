use crate::Message;
use anyhow::Context;

/// json format message parser
pub struct MessageParser {}

impl MessageParser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse(&self, message: &str) -> anyhow::Result<Message> {
        serde_json::from_str(message).context("message parse failed")
    }
}
