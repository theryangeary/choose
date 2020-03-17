use regex::Regex;
use std::num::ParseIntError;
use std::process;

use crate::choice::Choice;
use crate::opt::Opt;

lazy_static! {
    static ref PARSE_CHOICE_RE: Regex = Regex::new(r"^(\d*):(\d*)$").unwrap();
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

        let separator = match Regex::new(match &opt.field_separator {
            Some(s) => s,
            None => "[[:space:]]",
        }) {
            Ok(r) => r,
            Err(e) => {
                // Exit code of 2 means failed to compile field_separator regex
                match e {
                    regex::Error::Syntax(e) => {
                        eprintln!("Syntax error compiling regular expression: {}", e);
                        process::exit(2);
                    }
                    regex::Error::CompiledTooBig(e) => {
                        eprintln!("Compiled regular expression too big: compiled size cannot exceed {} bytes", e);
                        process::exit(2);
                    }
                    _ => {
                        eprintln!("Error compiling regular expression: {}", e);
                        process::exit(2);
                    }
                }
            }
        };

        Config { opt, separator }
    }

    pub fn parse_choice(src: &str) -> Result<Choice, ParseIntError> {
        let cap = match PARSE_CHOICE_RE.captures_iter(src).next() {
            Some(v) => v,
            None => match src.parse() {
                Ok(x) => return Ok(Choice::new(x, x)),
                Err(e) => {
                    eprintln!("failed to parse choice argument: {}", src);
                    return Err(e);
                }
            },
        };

        let start = if cap[1].is_empty() {
            usize::min_value()
        } else {
            match cap[1].parse() {
                Ok(x) => x,
                Err(e) => {
                    eprintln!("failed to parse range start: {}", &cap[1]);
                    return Err(e);
                }
            }
        };

        let end = if cap[2].is_empty() {
            usize::max_value()
        } else {
            match cap[2].parse() {
                Ok(x) => x,
                Err(e) => {
                    eprintln!("failed to parse range end: {}", &cap[2]);
                    return Err(e);
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

        #[test]
        fn parse_bad_choice() {
            assert!(Config::parse_choice("d").is_err());
        }

        #[test]
        fn parse_bad_range() {
            assert!(Config::parse_choice("d:i").is_err());
        }
    }
}
