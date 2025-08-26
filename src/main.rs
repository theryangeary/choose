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

fn main() {
    let opt = Opt::from_args();

    let stdout = io::stdout();
    let lock = stdout.lock();
    let exit_result = match opt.input {
        Some(_) => main_generic(opt, &mut io::BufWriter::new(lock)),
        None => main_generic(opt, &mut io::LineWriter::new(lock)),
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

fn main_generic<W: WriteReceiver>(opt: Opt, handle: &mut W) -> Result<()> {
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

    while let Some(line) = reader.read_line(&mut buffer) {
        match line {
            Ok(l) => {
                let l = if (config.opt.character_wise || config.opt.field_separator.is_some())
                    && l.ends_with('\n')
                {
                    &l[0..l.len().saturating_sub(1)]
                } else {
                    l
                };

                // trim end to remove newline or CRLF on windows
                let l = l.trim_end();

                let choice_iter = &mut config.opt.choices.iter().peekable();

                while let Some(choice) = choice_iter.next() {
                    choice.print_choice(l, &config, handle)?;
                    if choice_iter.peek().is_some() {
                        handle.write_separator(&config)?;
                    }
                }

                handle.write(b"\n").map(|_| ())?
            }
            Err(e) => println!("Failed to read line: {}", e),
        }
    }

    Ok(())
}
