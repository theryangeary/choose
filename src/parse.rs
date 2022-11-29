use std::str::FromStr;

use backslash::escape_ascii;
use regex::Regex;

use crate::choice::{Choice, ChoiceKind};
use crate::error::ParseRangeError;
use crate::parse_error::ParseError;

lazy_static! {
    static ref PARSE_CHOICE_RE: Regex = Regex::new(r"^(-?\d*)(:|\.\.=?)(-?\d*)$").unwrap();
}

impl FromStr for Choice {
    type Err = crate::parse_error::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        crate::parse::choice(s)
    }
}

pub fn choice(src: &str) -> Result<Choice, ParseError> {
    let cap = match PARSE_CHOICE_RE.captures_iter(src).next() {
        Some(v) => v,
        None => match src.parse() {
            Ok(x) => return Ok(Choice::new(x, x, ChoiceKind::Single)),
            Err(e) => {
                return Err(ParseError::ParseIntError(e));
            }
        },
    };

    let start = if cap[1].is_empty() {
        0
    } else {
        match cap[1].parse() {
            Ok(x) => x,
            Err(e) => {
                return Err(ParseError::ParseIntError(e));
            }
        }
    };

    let kind = match &cap[2] {
        ":" => ChoiceKind::ColonRange,
        ".." => ChoiceKind::RustExclusiveRange,
        "..=" => ChoiceKind::RustInclusiveRange,
        _ => {
            return Err(ParseError::ParseRangeError(ParseRangeError::new(&cap[2])));
        }
    };

    let end = if cap[3].is_empty() {
        isize::max_value()
    } else {
        match cap[3].parse() {
            Ok(x) => x,
            Err(e) => {
                return Err(ParseError::ParseIntError(e));
            }
        }
    };

    Ok(Choice::new(start, end, kind))
}

// bpaf's parse gives String by value so type signature needs to match
#[allow(clippy::needless_pass_by_value)]
pub fn output_field_separator(src: String) -> String {
    escape_ascii(&src).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::parse;

    mod parse_choice_tests {
        use super::*;

        #[test]
        fn parse_single_choice_start() {
            let result = parse::choice("6").unwrap();
            assert_eq!(6, result.start)
        }

        #[test]
        fn parse_single_choice_end() {
            let result = parse::choice("6").unwrap();
            assert_eq!(6, result.end)
        }

        #[test]
        fn parse_none_started_range() {
            let result = parse::choice(":5").unwrap();
            assert_eq!((0, 5), (result.start, result.end))
        }

        #[test]
        fn parse_none_terminated_range() {
            let result = parse::choice("5:").unwrap();
            assert_eq!((5, isize::max_value()), (result.start, result.end))
        }

        #[test]
        fn parse_full_range_pos_pos() {
            let result = parse::choice("5:7").unwrap();
            assert_eq!((5, 7), (result.start, result.end))
        }

        #[test]
        fn parse_full_range_neg_neg() {
            let result = parse::choice("-3:-1").unwrap();
            assert_eq!((-3, -1), (result.start, result.end))
        }

        #[test]
        fn parse_neg_started_none_ended() {
            let result = parse::choice("-3:").unwrap();
            assert_eq!((-3, isize::max_value()), (result.start, result.end))
        }

        #[test]
        fn parse_none_started_neg_ended() {
            let result = parse::choice(":-1").unwrap();
            assert_eq!((0, -1), (result.start, result.end))
        }

        #[test]
        fn parse_full_range_pos_neg() {
            let result = parse::choice("5:-3").unwrap();
            assert_eq!((5, -3), (result.start, result.end))
        }

        #[test]
        fn parse_full_range_neg_pos() {
            let result = parse::choice("-3:5").unwrap();
            assert_eq!((-3, 5), (result.start, result.end))
        }

        #[test]
        fn parse_beginning_to_end_range() {
            let result = parse::choice(":").unwrap();
            assert_eq!((0, isize::max_value()), (result.start, result.end))
        }

        #[test]
        fn parse_bad_choice() {
            assert!(parse::choice("d").is_err());
        }

        #[test]
        fn parse_bad_range() {
            assert!(parse::choice("d:i").is_err());
        }

        #[test]
        fn parse_rust_inclusive_range() {
            let result = parse::choice("3..=5").unwrap();
            assert_eq!((3, 5), (result.start, result.end))
        }

        #[test]
        fn parse_rust_inclusive_range_no_start() {
            let result = parse::choice("..=5").unwrap();
            assert_eq!((0, 5), (result.start, result.end))
        }

        #[test]
        fn parse_rust_inclusive_range_no_end() {
            let result = parse::choice("3..=").unwrap();
            assert_eq!((3, isize::max_value()), (result.start, result.end))
        }

        #[test]
        fn parse_rust_inclusive_range_no_start_or_end() {
            let result = parse::choice("..=").unwrap();
            assert_eq!((0, isize::max_value()), (result.start, result.end))
        }

        #[test]
        fn parse_full_range_pos_pos_rust_exclusive() {
            let result = parse::choice("5..7").unwrap();
            assert_eq!((5, 7), (result.start, result.end))
        }

        #[test]
        fn parse_full_range_neg_neg_rust_exclusive() {
            let result = parse::choice("-3..-1").unwrap();
            assert_eq!((-3, -1), (result.start, result.end))
        }

        #[test]
        fn parse_neg_started_none_ended_rust_exclusive() {
            let result = parse::choice("-3..").unwrap();
            assert_eq!((-3, isize::max_value()), (result.start, result.end))
        }

        #[test]
        fn parse_none_started_neg_ended_rust_exclusive() {
            let result = parse::choice("..-1").unwrap();
            assert_eq!((0, -1), (result.start, result.end))
        }

        #[test]
        fn parse_full_range_pos_neg_rust_exclusive() {
            let result = parse::choice("5..-3").unwrap();
            assert_eq!((5, -3), (result.start, result.end))
        }

        #[test]
        fn parse_full_range_neg_pos_rust_exclusive() {
            let result = parse::choice("-3..5").unwrap();
            assert_eq!((-3, 5), (result.start, result.end))
        }

        #[test]
        fn parse_rust_exclusive_range() {
            let result = parse::choice("3..5").unwrap();
            assert_eq!((3, 5), (result.start, result.end))
        }

        #[test]
        fn parse_rust_exclusive_range_no_start() {
            let result = parse::choice("..5").unwrap();
            assert_eq!((0, 5), (result.start, result.end))
        }

        #[test]
        fn parse_rust_exclusive_range_no_end() {
            let result = parse::choice("3..").unwrap();
            assert_eq!((3, isize::max_value()), (result.start, result.end))
        }

        #[test]
        fn parse_rust_exclusive_range_no_start_or_end() {
            let result = parse::choice("..").unwrap();
            assert_eq!((0, isize::max_value()), (result.start, result.end))
        }

        #[test]
        fn parse_full_range_pos_pos_rust_inclusive() {
            let result = parse::choice("5..=7").unwrap();
            assert_eq!((5, 7), (result.start, result.end))
        }

        #[test]
        fn parse_full_range_neg_neg_rust_inclusive() {
            let result = parse::choice("-3..=-1").unwrap();
            assert_eq!((-3, -1), (result.start, result.end))
        }

        #[test]
        fn parse_neg_started_none_ended_rust_inclusive() {
            let result = parse::choice("-3..=").unwrap();
            assert_eq!((-3, isize::max_value()), (result.start, result.end))
        }

        #[test]
        fn parse_none_started_neg_ended_rust_inclusive() {
            let result = parse::choice("..=-1").unwrap();
            assert_eq!((0, -1), (result.start, result.end))
        }

        #[test]
        fn parse_full_range_pos_neg_rust_inclusive() {
            let result = parse::choice("5..=-3").unwrap();
            assert_eq!((5, -3), (result.start, result.end))
        }

        #[test]
        fn parse_full_range_neg_pos_rust_inclusive() {
            let result = parse::choice("-3..=5").unwrap();
            assert_eq!((-3, 5), (result.start, result.end))
        }
    }
}
