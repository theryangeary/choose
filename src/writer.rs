use std::io::{self, BufWriter, LineWriter, Write};

use crate::config::Config;
use crate::writeable::Writeable;

pub trait WriteReceiver: Write {
    fn write_choice<Wa: Writeable>(
        &mut self,
        b: Wa,
        config: &Config,
        print_separator: bool,
    ) -> io::Result<()> {
        let num_bytes_written = self.write(&b.to_byte_buf())?;
        if num_bytes_written > 0 && print_separator {
            self.write_separator(config)?;
        };
        Ok(())
    }

    fn write_separator(&mut self, config: &Config) -> io::Result<()> {
        self.write(&config.output_separator).map(|_| ())
    }
}

impl<W: Write> WriteReceiver for BufWriter<W> {}

impl<W: Write> WriteReceiver for LineWriter<W> {}
