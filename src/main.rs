use regex::Regex;
use std::convert::TryInto;
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

impl Choice {
    fn print_choice(&self, line: &String, opt: &Opt) {
        let words: Vec<&str> = line.split_whitespace().collect();

        match self {
            Choice::Field(i) => print!("{} ", words[*i as usize]),
            Choice::FieldRange(r) => match r {
                (None, None) => print!("{}", words.into_iter().collect::<String>()),
                (Some(start), None) => print!(
                    "{} ",
                    words
                        .into_iter()
                        .enumerate()
                        .filter(|x| x.0 >= (*start).try_into().unwrap())
                        .map(|x| x.1)
                        .collect::<Vec<&str>>()
                        .join(" ")
                ),
                (None, Some(end)) => {
                    let e: usize = if opt.inclusive {
                        (end + 1).try_into().unwrap()
                    } else {
                        (*end).try_into().unwrap()
                    };
                    print!(
                        "{} ",
                        words
                            .into_iter()
                            .enumerate()
                            .filter(|x| x.0 < e)
                            .map(|x| x.1)
                            .collect::<Vec<&str>>()
                            .join(" ")
                    )
                }
                (Some(start), Some(end)) => {
                    let e: usize = if opt.inclusive {
                        (end + 1).try_into().unwrap()
                    } else {
                        (*end).try_into().unwrap()
                    };
                    print!(
                        "{} ",
                        words
                            .into_iter()
                            .enumerate()
                            .filter(|x| x.0 < e && x.0 >= (*start).try_into().unwrap())
                            .map(|x| x.1)
                            .collect::<Vec<&str>>()
                            .join(" ")
                    )
                }
            },
        };
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

    /// Fields to print. Either x, x:, :y, or x:y, where x and y are integers, colons indicate a
    /// range, and an empty field on either side of the colon continues to the beginning or end of
    /// the line.
    #[structopt(required = true, min_values = 1, parse(try_from_str = Choice::parse_choice))]
    choice: Vec<Choice>,
}

fn main() {
    let opt = Opt::from_args();

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
