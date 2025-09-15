use std::fs::File;
use std::io::{self, Read};
use std::process;
use structopt::StructOpt;

#[macro_use]
extern crate lazy_static;

mod choice;
mod config;
mod error;
mod escape;
mod opt;
mod parse;
mod parse_error;
mod reader;
mod result;
mod writeable;
mod writer;

use config::Config;
use error::Error;
use opt::Opt;
use result::Result;
use writer::WriteReceiver;

use crate::writer::Writer;

fn main() {
    let opt = Opt::from_args();

    let stdout = io::stdout();
    let lock = stdout.lock();
    let exit_result = match opt.input {
        Some(_) => main_generic(opt, Writer::from(io::BufWriter::new(lock))),
        None => main_generic(opt, Writer::from(io::LineWriter::new(lock))),
    };

    match exit_result {
        Ok(_) => (),
        Err(err) => {
            match err {
                Error::Io(e) => {
                    if e.kind() == io::ErrorKind::BrokenPipe {
                        // BrokenPipe means whoever is reading the output hung up, we should
                        // gracefully exit
                    } else {
                        eprintln!("Failed to write to output: {}", e)
                    }
                }
                e => eprintln!("Error: {}", e),
            }
        }
    }
}

fn main_generic<W: WriteReceiver>(opt: Opt, mut handle: Writer<W>) -> Result<()> {
    let config = Config::new(opt);

    let read = match &config.opt.input {
        Some(f) => match File::open(f) {
            Ok(fh) => Box::new(fh) as Box<dyn Read>,
            Err(e) => {
                eprintln!("Failed to open file: {}", e);
                // exit code of 3 means failure to open input file
                process::exit(3);
            }
        },
        None => Box::new(io::stdin()) as Box<dyn Read>,
    };

    let mut reader = reader::BufReader::new(read);
    let mut buffer = String::new();

    loop {
        match reader.read_line(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    // EOF
                    break;
                }

                let buffer_bytes = buffer.as_bytes();
                let mut last_index = buffer_bytes.len().saturating_sub(1);
                // slice off line feed if present
                if buffer_bytes[last_index] == b'\n' {
                    last_index = last_index.saturating_sub(1);
                }
                // slice off carriage return if present
                if buffer_bytes[last_index] == b'\r' {
                    last_index = last_index.saturating_sub(1);
                }
                let buffer_slice = &buffer[..=last_index];

                process_all_choices_for_line(&mut handle, &config, &buffer_slice)?;

                handle.write_line()?;
            }
            Err(e) => println!("Failed to read line: {}", e),
        }
    }

    Ok(())
}

fn process_all_choices_for_line<W: WriteReceiver>(
    handle: &mut Writer<W>,
    config: &Config,
    line: &str,
) -> Result<()> {
    Ok(for choice_index in 0..config.opt.choices.len() {
        config.opt.choices[choice_index].print_choice(line, config, handle)?;
    })
}
