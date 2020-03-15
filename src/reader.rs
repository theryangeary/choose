use std::io::{self, prelude::*};

pub struct BufReader<R> {
    reader: io::BufReader<R>,
}

impl<R: Read> BufReader<R> {
    pub fn new(f: R) -> Self {
        Self { reader: io::BufReader::new(f) }
    }

    pub fn read_line<'buf>(
        &mut self,
        buffer: &'buf mut String,
    ) -> Option<io::Result<&'buf mut String>> {
        buffer.clear();

        self.reader
            .read_line(buffer)
            .map(|u| if u == 0 { None } else { Some(buffer) })
            .transpose()
    }
}
