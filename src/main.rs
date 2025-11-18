use std::fs::File;
use std::io::{self, BufRead, Read};
use std::process;
use structopt::StructOpt;

#[macro_use]
extern crate lazy_static;

mod choice;
mod config;
mod error;
mod opt;
mod parse;
mod parse_error;
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
        // it is important to use a LineWriter instead of BufWriter so that if
        // the user is sitting waiting for output for a pipeline that gives lines
        // slowly, they won't have to wait for the buffer to fill up before they
        // see anything (think `tail`ing logs)
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

    let mut reader = io::BufReader::new(read);
    let mut buffer = String::new();

    loop {
        buffer.clear();
        match reader.read_line(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    // EOF
                    break;
                }

                if buffer.ends_with('\n') {
                    buffer.pop();
                    if buffer.ends_with('\r') {
                        buffer.pop();
                    }
                }

                process_all_choices_for_line(&mut handle, &config, &buffer)?;

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
    let _: () = for choice in &config.opt.choices {
        choice.print_choice(line, config, handle)?;
    };
    Ok(())
}
