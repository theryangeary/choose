use std::fs::File;
use std::io::{self, Read, Write};
use std::process;
use structopt::StructOpt;

#[macro_use]
extern crate lazy_static;

mod choice;
mod config;
mod errors;
mod opt;
mod parse;
mod parse_error;
mod reader;
mod writeable;
mod writer;
use config::Config;
use opt::Opt;
use writer::WriteReceiver;

fn main() {
    let opt = Opt::from_args();
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

    let stdout = io::stdout();
    let lock = stdout.lock();
    let mut handle = io::BufWriter::new(lock);

    while let Some(line) = reader.read_line(&mut buffer) {
        match line {
            Ok(l) => {
                let choice_iter = &mut config.opt.choices.iter().peekable();
                while let Some(choice) = choice_iter.next() {
                    choice.print_choice(&l, &config, &mut handle);
                    if choice_iter.peek().is_some() {
                        handle.write_separator(&config);
                    }
                }
                match handle.write(b"\n") {
                    Ok(_) => (),
                    Err(e) => eprintln!("Failed to write to output: {}", e),
                }
            }
            Err(e) => println!("Failed to read line: {}", e),
        }
    }
}
