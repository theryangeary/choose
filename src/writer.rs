use std::cell::RefCell;
use std::io::{self, BufWriter, LineWriter, Write};

use crate::config::Config;
use crate::writeable::Writeable;

pub struct Writer<WR: WriteReceiver> {
    first_of_line: RefCell<bool>,
    pub inner: Box<WR>,
}

pub trait WriteReceiver: Write {
    /// write_choice prints the Writable, followed by the output separator (if
    /// print_separator is true)
    ///
    /// This means the caller must know if there will be another Writeable on
    /// the current line. To avoid the need to look forward, use
    /// write_choice_separable
    fn write_choice<Wa: Writeable>(
        &mut self,
        b: Wa,
        config: &Config,
        print_separator: bool,
    ) -> io::Result<()> {
        let num_bytes_written = self.write(&b.as_bytes())?;
        if num_bytes_written > 0 && print_separator {
            self.write_separator(config)?;
        };
        Ok(())
    }

    /// write_choice_separable prints the output separator (if first is not
    /// true) followed by the Writeable
    ///
    /// when looping through a series of Writeables, this achieves the same
    /// things as write_choice, but requires the caller to track `first`. This
    /// additional overhead on the caller allows for performance gains by
    /// removing look-ahead capabilities on the caller side.
    /// 
    /// [Writer] is provided as a convenience for tracking `first`.
    fn write_choice_separable<Wa: Writeable>(
        &mut self,
        b: Wa,
        config: &Config,
        first: bool,
    ) -> io::Result<()> {
        if !first && !b.is_empty() {
            self.write_separator(config)?;
        }
        self.write(&b.as_bytes())?;
        Ok(())
    }

    fn write_separator(&mut self, config: &Config) -> io::Result<()> {
        self.write(&config.output_separator).map(|_| ())
    }
}

impl<W: Write> WriteReceiver for BufWriter<W> {}

impl<W: Write> WriteReceiver for LineWriter<W> {}

impl<WR: WriteReceiver> From<WR> for Writer<WR> {
    fn from(wr: WR) -> Self {
        Self {
            first_of_line: RefCell::from(true),
            inner: Box::from(wr),
        }
    }
}

impl<WR: WriteReceiver> Writer<WR> {
    pub fn write_choice<Wa: Writeable>(
        &mut self,
        b: Wa,
        config: &Config,
        print_separator: bool,
    ) -> io::Result<()> {
        WR::write_choice(&mut self.inner, b, config, print_separator)?;
        self.first_of_line.replace(false);
        Ok(())
    }

    pub fn write_choice_separable<Wa: Writeable>(
        &mut self,
        b: Wa,
        config: &Config,
    ) -> io::Result<()> {
        WR::write_choice_separable(&mut self.inner, b, config, *self.first_of_line.borrow())?;
        self.first_of_line.replace(false);
        Ok(())
    }

    pub fn write_line(&mut self) -> io::Result<()> {
        WR::write(&mut self.inner, b"\n").map(|_| ())?;
        self.first_of_line.replace(true);
        Ok(())
    }

    /// into_inner decomposes a Writer into its underlying WriteReceiver,
    /// convenient for testing
    #[allow(unused)]
    pub fn into_inner(self) -> WR {
        *self.inner
    }
}
