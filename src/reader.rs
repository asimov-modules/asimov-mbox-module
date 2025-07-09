// This is free and unencumbered software released into the public domain.

use super::{MboxIterator, MboxMessage};
use core::result::Result;
use know::datatypes::EmailMessageId;
use mailparse::MailParseError;
use mbox_reader::MboxFile;
use std::{io, path::Path};

pub struct MboxReader {
    mbox: MboxFile,
}

impl MboxReader {
    pub fn open(path: impl AsRef<Path>) -> io::Result<Self> {
        Ok(Self {
            mbox: MboxFile::from_file(path.as_ref())?,
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = Result<MboxMessage, MailParseError>> {
        MboxIterator::new(self.mbox.iter())
    }

    pub fn fetch(&self, mid: &EmailMessageId) -> Result<Option<MboxMessage>, MailParseError> {
        for entry in self.iter() {
            let message = entry?;
            if let Some(message_id) = message.message.id.as_ref() {
                if message_id == mid {
                    return Ok(Some(message));
                }
            }
        }
        Ok(None)
    }
}
