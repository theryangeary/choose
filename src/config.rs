use regex::Regex;
use std::num::ParseIntError;
use std::path::PathBuf;
use std::process;
use structopt::StructOpt;

use crate::choice::Choice;

#[derive(Debug, StructOpt)]
#[structopt(name = "choose", about = "`choose` sections from each line of files")]
pub struct Opt {
    /// Specify field separator other than whitespace
    #[structopt(short, long)]
    pub field_separator: Option<String>,

    /// Use exclusive ranges, similar to array slicing in many programming languages
    #[structopt(short = "x", long)]
    pub exclusive: bool,

    /// Activate debug mode
    #[structopt(short, long)]
    pub debug: bool,

    /// Input file
    #[structopt(short, long, parse(from_os_str))]
    pub input: Option<PathBuf>,

    /// Fields to print. Either x, x:, :y, or x:y, where x and y are integers, colons indicate a
    /// range, and an empty field on either side of the colon continues to the beginning or end of
    /// the line.
    #[structopt(required = true, min_values = 1, parse(try_from_str = Config::parse_choice))]
    pub choice: Vec<Choice>,
}

pub struct Config {
    pub opt: Opt,
    pub separator: Regex,
}

impl Config {
    pub fn new(mut opt: Opt) -> Self {
        if opt.exclusive {
            for mut choice in &mut opt.choice {
                if choice.is_reverse_range() {
                    choice.start = choice.start - 1;
                } else {
                    choice.end = choice.end - 1;
                }
            }
        }

        let separator = Regex::new(match &opt.field_separator {
            Some(s) => s,
            None => "[[:space:]]",
        })
        .unwrap_or_else(|e| {
            eprintln!("Failed to compile regular expression: {}", e);
            // Exit code of 1 means failed to compile field_separator regex
            process::exit(1);
        });
        Config { opt, separator }
    }

    pub fn parse_choice(src: &str) -> Result<Choice, ParseIntError> {
        let re = Regex::new(r"^(\d*):(\d*)$").unwrap();

        let cap = match re.captures_iter(src).next() {
            Some(v) => v,
            None => match src.parse() {
                Ok(x) => return Ok(Choice::new(x, x)),
                Err(_) => {
                    eprintln!("failed to parse choice argument: {}", src);
                    // Exit code of 2 means failed to parse choice argument
                    process::exit(2);
                }
            },
        };

        let start = if cap[1].is_empty() {
            usize::min_value()
        } else {
            match cap[1].parse() {
                Ok(x) => x,
                Err(_) => {
                    eprintln!("failed to parse range start: {}", &cap[1]);
                    process::exit(2);
                }
            }
        };

        let end = if cap[2].is_empty() {
            usize::max_value()
        } else {
            match cap[2].parse() {
                Ok(x) => x,
                Err(_) => {
                    eprintln!("failed to parse range end: {}", &cap[2]);
                    process::exit(2);
                }
            }
        };

        return Ok(Choice::new(start, end));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod parse_choice_tests {
        use super::*;

        #[test]
        fn parse_single_choice_start() {
            let result = Config::parse_choice("6").unwrap();
            assert_eq!(6, result.start)
        }

        #[test]
        fn parse_single_choice_end() {
            let result = Config::parse_choice("6").unwrap();
            assert_eq!(6, result.end)
        }

        #[test]
        fn parse_none_started_range() {
            let result = Config::parse_choice(":5").unwrap();
            assert_eq!((usize::min_value(), 5), (result.start, result.end))
        }

        #[test]
        fn parse_none_terminated_range() {
            let result = Config::parse_choice("5:").unwrap();
            assert_eq!((5, usize::max_value()), (result.start, result.end))
        }

        #[test]
        fn parse_full_range() {
            let result = Config::parse_choice("5:7").unwrap();
            assert_eq!((5, 7), (result.start, result.end))
        }

        #[test]
        fn parse_beginning_to_end_range() {
            let result = Config::parse_choice(":").unwrap();
            assert_eq!(
                (usize::min_value(), usize::max_value()),
                (result.start, result.end)
            )
        }

        // These tests should pass once parse_choice return errors properly, but until that time
        // makes running other tests impossible.
        //#[test]
        //fn parse_bad_choice() {
        //assert!(Config::parse_choice("d").is_err());
        //}

        //#[test]
        //fn parse_bad_range() {
        //assert!(Config::parse_choice("d:i").is_err());
        //}
    }
}
