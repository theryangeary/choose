use std::io::{BufWriter, LineWriter, Write};

use crate::config::Config;
use crate::writeable::Writeable;

pub trait WriteReceiver: Write {
    fn write_choice<Wa: Writeable>(&mut self, b: Wa, config: &Config, print_separator: bool) {
        let num_bytes_written = match self.write(&b.to_byte_buf()) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to write to output: {}", e);
                0
            }
        };
        if num_bytes_written > 0 && print_separator {
            self.write_separator(config);
        };
    }

    fn write_separator(&mut self, config: &Config) {
        match self.write(&config.output_separator) {
            Ok(_) => (),
            Err(e) => eprintln!("Failed to write to output: {}", e),
        }
    }
}

impl<W: Write> WriteReceiver for BufWriter<W> {}

impl<W: Write> WriteReceiver for LineWriter<W> {}
