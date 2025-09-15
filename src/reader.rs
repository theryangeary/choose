use std::io::{self, prelude::*};

pub struct BufReader<R> {
    reader: io::BufReader<R>,
}

impl<R: Read> BufReader<R> {
    pub fn new(f: R) -> Self {
        Self {
            reader: io::BufReader::new(f),
        }
    }

    pub fn read_line<'buf>(
        &mut self,
        buffer: &'buf mut String,
    ) -> io::Result<usize> {
        buffer.clear();

        self.reader
            .read_line(buffer)
    }
}
