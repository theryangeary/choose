use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::path::PathBuf;
use std::num::ParseIntError;
use structopt::StructOpt;
use regex::Regex;

type Range = (Option<u32>, Option<u32>);

enum Choice {
    One(u32),
    Range
}

#[derive(Debug, StructOpt)]
#[structopt(name = "choose", about = "`choose` sections from each line of files")]
struct Opt {
    /// Capture range of fields
    #[structopt(parse(try_from_str = parse_range))]
    range: Range,

    /// Specify field separator other than whitespace
    #[structopt(short, long)]
    field_separator: Option<String>,

    /// Use inclusive ranges
    #[structopt(short, long)]
    inclusive: bool,

    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool,

    /// Input file
    #[structopt(parse(from_os_str))]
    input: Option<PathBuf>,
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

fn parse_range(src: &str) -> Result<Range, ParseIntError> {
    let re = Regex::new(r"^(\d*):(\d*)$").unwrap();

    let cap = match re.captures_iter(src).next() {
        Some(v) => v,
        None => panic!("failed to parse range argument: {}", src),
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

    return Ok((start, end));
}
