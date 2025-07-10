// This is free and unencumbered software released into the public domain.

use super::MboxError;
use know::classes::EmailMessage;
use mail_parser::MessageParser;
use mbox_reader::Entry;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MboxMessage {
    pub offset: usize,
    pub headers: EmailMessage,
    pub body: Option<String>,
}

impl TryFrom<Entry<'_>> for MboxMessage {
    type Error = MboxError;

    fn try_from(entry: Entry<'_>) -> Result<Self, Self::Error> {
        let bytes = entry.message().ok_or(MboxError::InvalidEntry)?;
        let message = MessageParser::default()
            .parse(bytes)
            .ok_or(MboxError::InvalidMessage)?;

        Ok(Self {
            offset: entry.offset(),
            headers: (&message)
                .try_into()
                .map_err(|_| MboxError::InvalidHeaders)?,
            body: message.body_text(0).map(|s| s.into_owned()),
        })
    }
}
