// This is free and unencumbered software released into the public domain.

use super::MboxMessage;
use core::result::Result;
use mailparse::MailParseError;
use mbox_reader::MboxReader;

pub struct MboxIterator<'a>(MboxReader<'a>);

impl<'a> MboxIterator<'a> {
    pub fn new(inner: MboxReader<'a>) -> Self {
        Self(inner)
    }
}

impl Iterator for MboxIterator<'_> {
    type Item = Result<MboxMessage, MailParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|entry| entry.try_into())
    }
}
