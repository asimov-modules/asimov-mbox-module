// This is free and unencumbered software released into the public domain.

use mailparse::MailHeaderMap;
use mbox_reader::Entry;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MboxMessage {
    pub id: String,
}

impl TryFrom<Entry<'_>> for MboxMessage {
    type Error = mailparse::MailParseError;

    fn try_from(entry: Entry<'_>) -> Result<Self, Self::Error> {
        let bytes = entry.message().unwrap(); // safe
        let msg = mailparse::parse_mail(bytes)?;
        let id = msg
            .headers
            .get_first_value("Message-ID")
            .unwrap_or_default()
            .trim()
            .to_string();
        Ok(Self { id })
    }
}
