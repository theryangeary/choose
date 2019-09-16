#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_single_choice() {
        let result = Choice::parse_choice("6").unwrap();
        assert_eq!(
            6,
            match result {
                Choice::Field(x) => x,
                _ => panic!(),
            }
        )
    }

    #[test]
    fn parse_none_started_range() {
        let result = Choice::parse_choice(":5").unwrap();
        assert_eq!(
            (None, Some(5)),
            match result {
                Choice::FieldRange(x) => x,
                _ => panic!(),
            }
        )
    }

    #[test]
    fn parse_none_terminated_range() {
        let result = Choice::parse_choice("5:").unwrap();
        assert_eq!(
            (Some(5), None),
            match result {
                Choice::FieldRange(x) => x,
                _ => panic!(),
            }
        )
    }

    #[test]
    fn parse_full_range() {
        let result = Choice::parse_choice("5:7").unwrap();
        assert_eq!(
            (Some(5), Some(7)),
            match result {
                Choice::FieldRange(x) => x,
                _ => panic!(),
            }
        )
    }

    #[test]
    fn parse_beginning_to_end_range() {
        let result = Choice::parse_choice(":").unwrap();
        assert_eq!(
            (None, None),
            match result {
                Choice::FieldRange(x) => x,
                _ => panic!(),
            }
        )
    }

}

use regex::Regex;
use std::convert::TryInto;
use std::num::ParseIntError;
use std::path::PathBuf;
use std::process;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "choose", about = "`choose` sections from each line of files")]
pub struct Opt {
    /// Specify field separator other than whitespace
    #[structopt(short, long)]
    pub field_separator: Option<String>,

    /// Use inclusive ranges
    #[structopt(short = "n", long)]
    pub inclusive: bool,

    /// Activate debug mode
    #[structopt(short, long)]
    pub debug: bool,

    /// Input file
    #[structopt(short, long, parse(from_os_str))]
    pub input: Option<PathBuf>,

    /// Fields to print. Either x, x:, :y, or x:y, where x and y are integers, colons indicate a
    /// range, and an empty field on either side of the colon continues to the beginning or end of
    /// the line.
    #[structopt(required = true, min_values = 1, parse(try_from_str = Choice::parse_choice))]
    pub choice: Vec<Choice>,
}

pub type Range = (Option<u32>, Option<u32>);

#[derive(Debug)]
pub enum Choice {
    Field(u32),
    FieldRange(Range),
}

impl Choice {
    pub fn print_choice(&self, line: &String, opt: &Opt) {
        let re = Regex::new(match &opt.field_separator {
            Some(s) => s,
            None => "[[:space:]]",
        })
        .unwrap_or_else(|e| {
            eprintln!("Failed to compile regular expression: {}", e);
            // Exit code of 1 means failed to compile field_separator regex
            process::exit(1);
        });

        let words = re
            .split(line)
            .into_iter()
            .filter(|s| !s.is_empty())
            .enumerate();

        match self {
            Choice::Field(i) => {
                print!(
                    "{} ",
                    words
                        .filter(|x| x.0 == *i as usize)
                        .map(|x| x.1)
                        .collect::<String>()
                );
            }
            Choice::FieldRange(r) => match r {
                (None, None) => print!("{}", words.map(|x| x.1).collect::<String>()),
                (Some(start), None) => print!(
                    "{} ",
                    words
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
                            .filter(|x| x.0 < e && x.0 >= (*start).try_into().unwrap())
                            .map(|x| x.1)
                            .collect::<Vec<&str>>()
                            .join(" ")
                    )
                }
            },
        };
    }

    pub fn parse_choice(src: &str) -> Result<Choice, ParseIntError> {
        let re = Regex::new(r"^(\d*):(\d*)$").unwrap();

        let cap = match re.captures_iter(src).next() {
            Some(v) => v,
            None => match src.parse() {
                Ok(x) => return Ok(Choice::Field(x)),
                Err(_) => {
                    eprintln!("failed to parse choice argument: {}", src);
                    // Exit code of 2 means failed to parse choice argument
                    process::exit(2);
                }
            },
        };

        let start = if cap[1].is_empty() {
            None
        } else {
            match cap[1].parse() {
                Ok(x) => Some(x),
                Err(_) => {
                    eprintln!("failed to parse range start: {}", &cap[1]);
                    process::exit(2);
                }
            }
        };

        let end = if cap[2].is_empty() {
            None
        } else {
            match cap[2].parse() {
                Ok(x) => Some(x),
                Err(_) => {
                    eprintln!("failed to parse range end: {}", &cap[2]);
                    process::exit(2);
                }
            }
        };

        return Ok(Choice::FieldRange((start, end)));
    }
}
