use std::fs::File;
use std::io::{self, Read, Write};
use structopt::StructOpt;

mod choice;
mod config;
mod reader;
use config::Config;

fn main() {
    let opt = config::Opt::from_args();
    let config = Config::new(opt);

    let read = match &config.opt.input {
        Some(f) => Box::new(File::open(f).expect("Could not open file")) as Box<dyn Read>,
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
                for choice in &config.opt.choice {
                    choice.print_choice(&l, &config, &mut handle);
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
