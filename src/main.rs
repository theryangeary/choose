use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::process;
use structopt::StructOpt;

mod choice;

fn main() {
    let opt = choice::Opt::from_args();

    let read = match &opt.input {
        Some(f) => Box::new(File::open(f).expect("Could not open file")) as Box<dyn Read>,
        None => Box::new(io::stdin()) as Box<dyn Read>,
    };

    let buf = BufReader::new(read);

    let stdout = io::stdout();
    let lock = stdout.lock();
    let mut handle = io::BufWriter::new(lock);

    let re = Regex::new(match &opt.field_separator {
        Some(s) => s,
        None => "[[:space:]]",
    })
    .unwrap_or_else(|e| {
        eprintln!("Failed to compile regular expression: {}", e);
        // Exit code of 1 means failed to compile field_separator regex
        process::exit(1);
    });

    let lines = buf.lines();
    for line in lines {
        match line {
            Ok(l) => {
                for choice in &opt.choice {
                    choice.print_choice(&l, &opt, &re, &mut handle);
                }
                writeln!(handle, "");
            }
            Err(e) => println!("ERROR: {}", e),
        }
    }
}
