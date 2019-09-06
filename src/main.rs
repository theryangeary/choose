use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::num::ParseIntError;
use std::path::PathBuf;
use structopt::StructOpt;

type Range = (Option<u32>, Option<u32>);

#[derive(Debug)]
enum Choice {
    Field(u32),
    FieldRange(Range),
}

#[derive(Debug, StructOpt)]
#[structopt(name = "choose", about = "`choose` sections from each line of files")]
struct Opt {
    /// Specify field separator other than whitespace
    #[structopt(short, long)]
    field_separator: Option<String>,

    /// Use inclusive ranges
    #[structopt(short = "n", long)]
    inclusive: bool,

    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool,

    /// Input file
    #[structopt(short, long, parse(from_os_str))]
    input: Option<PathBuf>,

    /// Fields to print
    #[structopt(required = true, min_values = 1, parse(try_from_str = parse_choice))]
    choice: Vec<Choice>,
}

fn main() {
    let opt = Opt::from_args();

    let read = match &opt.input {
        Some(f) => Box::new(File::open(f).expect("Could not open file")) as Box<dyn Read>,
        None => Box::new(io::stdin()) as Box<dyn Read>,
    };

    let buf = BufReader::new(read);

    for line in buf.lines() {
        println!("{}", line.unwrap());
    }

    println!("Hello, world!");
}

fn parse_choice(src: &str) -> Result<Choice, ParseIntError> {
    let re = Regex::new(r"^(\d*):(\d*)$").unwrap();

    let cap = match re.captures_iter(src).next() {
        Some(v) => v,
        None => match src.parse() {
            Ok(x) => return Ok(Choice::Field(x)),
            Err(_) => panic!("failed to parse range argument: {}", src),
        },
    };

    let start = if cap[1].is_empty() {
        None
    } else {
        match cap[1].parse() {
            Ok(x) => Some(x),
            Err(e) => panic!("failed to get range argument: {:?}", e),
        }
    };

    let end = if cap[2].is_empty() {
        None
    } else {
        match cap[2].parse() {
            Ok(x) => Some(x),
            Err(e) => panic!("failed to get range argument: {:?}", e),
        }
    };

    return Ok(Choice::FieldRange((start, end)));
}
