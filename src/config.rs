use regex::Regex;
use std::process;

use crate::choice::ChoiceKind;
use crate::opt::Opt;

pub struct Config {
    pub opt: Opt,
    pub separator: Regex,
    pub output_separator: Box<[u8]>,
}

impl Config {
    pub fn new(mut opt: Opt) -> Self {
        for choice in &mut opt.choices {
            if (opt.exclusive && choice.kind == ChoiceKind::ColonRange)
                || choice.kind == ChoiceKind::RustExclusiveRange
            {
                if choice.is_reverse_range() {
                    choice.start -= 1;
                } else {
                    choice.end -= 1;
                }
            }

            if opt.one_indexed {
                if choice.start > 0 {
                    choice.start -= 1;
                }

                if choice.end > 0 {
                    choice.end -= 1;
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

        let output_separator = match opt.character_wise {
            false => match opt.output_field_separator.clone() {
                Some(s) => s.into_boxed_str().into_boxed_bytes(),
                None => Box::new([0x20; 1]),
            },
            true => match opt.output_field_separator.clone() {
                Some(s) => s.into_boxed_str().into_boxed_bytes(),
                None => Box::new([]),
            },
        };

        Config {
            opt,
            separator,
            output_separator,
        }
    }
}
