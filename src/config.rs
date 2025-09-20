use regex::Regex;
use std::process;

use crate::choice::ChoiceKind;
use crate::opt::Opt;

pub enum Separator {
    Whitespace,
    LiteralChar(char),
    Regex(Regex),
}

pub struct Config {
    pub opt: Opt,
    pub separator: Separator,
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

        let separator = match &opt.field_separator {
            Some(s) => {
                match regex_is_literal_char(s) {
                    Some(c) => Separator::LiteralChar(c),
                    None => match Regex::new(s) {
                        Ok(r) => Separator::Regex(r),
                        Err(e) => {
                            // Exit code of 2 means failed to compile field_separator regex
                            match e {
                                regex::Error::Syntax(e) => {
                                    eprintln!("Syntax error compiling regular expression: {}", e);
                                    process::exit(2);
                                }
                                regex::Error::CompiledTooBig(e) => {
                                    eprintln!(
                                        "Compiled regular expression too big: compiled size cannot exceed {} bytes",
                                        e
                                    );
                                    process::exit(2);
                                }
                                _ => {
                                    eprintln!("Error compiling regular expression: {}", e);
                                    process::exit(2);
                                }
                            }
                        }
                    },
                }
            }
            None => Separator::Whitespace,
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

/// regex_is_literal_char determines if a given regex pattern would match only
/// literal instances of a specific character and if so returns that char
///
/// This is an optimization to avoid using regex when it gains no functionality
/// (vs literal char comparisons), so false negatives are acceptable (but incur
/// a performance penalty) where as false positives would break field
/// separation.
fn regex_is_literal_char(s: &str) -> Option<char> {
    if s.len() == 1 {
        let c = s.chars().next().expect("len is 1");
        if c.is_alphanumeric() {
            return Some(c);
        }
    }

    match s {
        // chars without special regex significance
        " " => Some(' '),
        "\\t" => Some('\t'),
        "!" => Some('!'),
        "@" => Some('@'),
        "#" => Some('#'),
        "%" => Some('%'),
        "&" => Some('&'),
        "," => Some(','),
        ":" => Some(':'),
        ";" => Some(';'),
        "<" => Some('<'),
        ">" => Some('>'),
        // chars where, when escaped, become literals
        "\\^" => Some('^'),
        "\\$" => Some('$'),
        "\\\\" => Some('\\'),
        "\\." => Some('.'),
        "\\*" => Some('*'),
        "\\(" => Some('('),
        "\\)" => Some(')'),
        "\\{" => Some('{'),
        "\\}" => Some('}'),
        "\\[" => Some('['),
        "\\]" => Some(']'),
        "\\|" => Some('|'),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_regex_is_literal_char_some(s: &str, c: char) {
        assert_eq!(regex_is_literal_char(s).unwrap(), c);
    }

    fn test_regex_is_literal_char_none(s: &str) {
        assert!(regex_is_literal_char(s).is_none());
    }

    #[test]
    fn test_regex_is_literal_char_alpha() {
        test_regex_is_literal_char_some("a", 'a');
        test_regex_is_literal_char_some("b", 'b');
        test_regex_is_literal_char_some("y", 'y');
        test_regex_is_literal_char_some("z", 'z');
        test_regex_is_literal_char_some("A", 'A');
        test_regex_is_literal_char_some("B", 'B');
        test_regex_is_literal_char_some("Y", 'Y');
        test_regex_is_literal_char_some("Z", 'Z');

        test_regex_is_literal_char_none("ab");
        test_regex_is_literal_char_none("a|b");
        test_regex_is_literal_char_none("a.");
    }

    #[test]
    fn test_regex_is_literal_char_numeric() {
        test_regex_is_literal_char_some("1", '1');
        test_regex_is_literal_char_some("2", '2');
        test_regex_is_literal_char_some("3", '3');
        test_regex_is_literal_char_some("9", '9');

        test_regex_is_literal_char_none("10");
        test_regex_is_literal_char_none("1|");
        test_regex_is_literal_char_none("1.");
    }

    #[test]
    fn test_symbols() {
        // explicitly test the symbol mapping, as mapping wrong will lead to
        // _very_ confusing behavior
        test_regex_is_literal_char_some(" ", ' ');
        test_regex_is_literal_char_some("\\t", '\t');
        test_regex_is_literal_char_some("!", '!');
        test_regex_is_literal_char_some("@", '@');
        test_regex_is_literal_char_some("#", '#');
        test_regex_is_literal_char_some("%", '%');
        test_regex_is_literal_char_some("&", '&');
        test_regex_is_literal_char_some(",", ',');
        test_regex_is_literal_char_some(":", ':');
        test_regex_is_literal_char_some(";", ';');
        test_regex_is_literal_char_some("<", '<');
        test_regex_is_literal_char_some(">", '>');

        test_regex_is_literal_char_some("\\^" ,'^');
        test_regex_is_literal_char_some("\\$" ,'$');
        test_regex_is_literal_char_some("\\\\",'\\');
        test_regex_is_literal_char_some("\\." ,'.');
        test_regex_is_literal_char_some("\\*" ,'*');
        test_regex_is_literal_char_some("\\(" ,'(');
        test_regex_is_literal_char_some("\\)" ,')');
        test_regex_is_literal_char_some("\\{" ,'{');
        test_regex_is_literal_char_some("\\}" ,'}');
        test_regex_is_literal_char_some("\\[" ,'[');
        test_regex_is_literal_char_some("\\]" ,']');
        test_regex_is_literal_char_some("\\|" ,'|');

        test_regex_is_literal_char_none("^");
        test_regex_is_literal_char_none("$");
        test_regex_is_literal_char_none(".");
        test_regex_is_literal_char_none("(");
        test_regex_is_literal_char_none(")");
        test_regex_is_literal_char_none("[");
        test_regex_is_literal_char_none("]");
        test_regex_is_literal_char_none("{");
        test_regex_is_literal_char_none("}");
    }
}
