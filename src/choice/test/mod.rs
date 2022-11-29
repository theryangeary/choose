use crate::config::Config;
use std::io::{self, BufWriter, Write};

mod get_negative_start_end;
mod is_reverse_range;
mod print_choice;

impl Config {
    pub fn from_args(items: &[&str]) -> Self {
        let items = bpaf::Args::from(items);
        Config::new(crate::opt::options().run_inner(items).unwrap())
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

    fn str_from_buf_writer(b: BufWriter<MockStdout>) -> String {
        match b.into_inner() {
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
