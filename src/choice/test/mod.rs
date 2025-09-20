use crate::config::Config;
use crate::opt::Opt;
use crate::writer::Writer;
use std::ffi::OsString;
use std::io::{self, BufWriter, Write};
use structopt::StructOpt;

mod get_negative_start_end;
mod is_reverse_range;
mod print_choice;
mod literal_separators;

impl Config {
    pub fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<OsString> + Clone,
    {
        Config::new(Opt::from_iter(iter))
    }
}

struct MockStdout {
    pub buffer: String,
}

impl MockStdout {
    fn new() -> Self {
        MockStdout {
            buffer: String::new(),
        }
    }

    fn str_from_writer(b: Writer<BufWriter<MockStdout>>) -> String {
        match b.into_inner().into_inner() {
            Ok(b) => b.buffer,
            Err(_) => panic!("Failed to access BufWriter inner writer"),
        }
        .trim_end()
        .to_string()
    }
}

impl Write for MockStdout {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut bytes_written = 0;
        for i in buf {
            self.buffer.push(*i as char);
            bytes_written += 1;
        }
        Ok(bytes_written)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
