// This is free and unencumbered software released into the public domain.

use know::classes::EmailMessage;
use mailparse::MailParseError;
use mbox_reader::Entry;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MboxMessage {
    pub pos: usize,
    pub message: EmailMessage,
}

impl TryFrom<Entry<'_>> for MboxMessage {
    type Error = MailParseError;

    fn try_from(entry: Entry<'_>) -> Result<Self, Self::Error> {
        let bytes = entry.message().unwrap(); // infallible
        let parsed_mail = mailparse::parse_mail(bytes)?;
        Ok(Self {
            pos: entry.offset(),
            message: (&parsed_mail).try_into()?,
        })
    }
}
