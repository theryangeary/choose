use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use structopt::StructOpt;

mod choice;

fn main() {
    let opt = choice::Opt::from_args();

    let read = match &opt.input {
        Some(f) => Box::new(File::open(f).expect("Could not open file")) as Box<dyn Read>,
        None => Box::new(io::stdin()) as Box<dyn Read>,
    };

    let buf = BufReader::new(read);

    let lines: Vec<String> = buf.lines().map(|x| x.unwrap()).collect();
    for line in lines {
        for choice in &opt.choice {
            choice.print_choice(&line, &opt);
        }
        println!();
    }
}
