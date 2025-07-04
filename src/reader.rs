// This is free and unencumbered software released into the public domain.

use super::{MboxIterator, MboxMessage};
use core::result::Result;
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

    pub fn iter(&self) -> impl Iterator<Item = Result<MboxMessage, mailparse::MailParseError>> {
        MboxIterator::new(self.mbox.iter())
    }
}
