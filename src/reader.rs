// This is free and unencumbered software released into the public domain.

use super::{MboxError, MboxIterator, MboxMessage};
use core::result::Result;
use know::datatypes::EmailMessageId;
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

    pub fn iter(&self) -> impl Iterator<Item = Result<MboxMessage, MboxError>> {
        MboxIterator::new(self.mbox.iter())
    }

    pub fn fetch(&self, message_id: &EmailMessageId) -> Result<Option<MboxMessage>, MboxError> {
        for entry in self.iter() {
            let message = entry?;
            if let Some(mid) = message.headers.id.as_ref() {
                if mid == message_id {
                    return Ok(Some(message));
                }
            }
        }
        Ok(None)
    }
}
