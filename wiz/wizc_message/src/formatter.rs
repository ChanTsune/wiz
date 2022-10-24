use crate::Message;

pub enum MessageFormat {
    Normal,
    Json,
}

pub struct MessageFormatter(MessageFormat);

impl MessageFormatter {
    const DEFAULT: Self = Self::new(MessageFormat::Normal);
    const JSON: Self = Self::new(MessageFormat::Json);

    const fn new(format: MessageFormat) -> Self {
        Self(format)
    }

    pub fn format(&self, message: Message) -> String {
        match self.0 {
            MessageFormat::Normal => message.to_string(),
            MessageFormat::Json => serde_json::to_string(&message).unwrap_or_default(),
        }
    }
}
