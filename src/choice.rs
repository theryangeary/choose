use std::convert::TryInto;
use std::iter::FromIterator;

use crate::config::Config;
use crate::writeable::Writeable;
use crate::writer::WriteReceiver;

#[derive(Debug)]
pub struct Choice {
    pub start: isize,
    pub end: isize,
    pub kind: ChoiceKind,
    negative_index: bool,
    reversed: bool,
}

#[derive(Debug, PartialEq)]
pub enum ChoiceKind {
    Single,
    RustExclusiveRange,
    RustInclusiveRange,
    ColonRange,
}

impl Choice {
    pub fn new(start: isize, end: isize, kind: ChoiceKind) -> Self {
        let negative_index = start < 0 || end < 0;
        let reversed = end < start && !(start >= 0 && end < 0);
        Choice {
            start,
            end,
            kind,
            negative_index,
            reversed,
        }
    }

    pub fn print_choice<W: WriteReceiver>(&self, line: &String, config: &Config, handle: &mut W) {
        if config.opt.character_wise {
            let line_chars = line[0..line.len() - 1].chars();
            self.print_choice_generic(line_chars, config, handle);
        } else {
            let line_iter = config
                .separator
                .split(line)
                .filter(|s| !s.is_empty() || config.opt.non_greedy);
            self.print_choice_generic(line_iter, config, handle);
        }
    }

    pub fn is_reverse_range(&self) -> bool {
        self.reversed
    }

    pub fn has_negative_index(&self) -> bool {
        self.negative_index
    }

    fn print_choice_generic<W, T, I>(&self, mut iter: I, config: &Config, handle: &mut W)
    where
        W: WriteReceiver,
        T: Writeable,
        I: Iterator<Item = T>,
    {
        if self.is_reverse_range() && !self.has_negative_index() {
            self.print_choice_reverse(iter, config, handle);
        } else if self.has_negative_index() {
            self.print_choice_negative(iter, config, handle);
        } else {
            if self.start > 0 {
                iter.nth((self.start - 1).try_into().unwrap());
            }
            let range = self.end.checked_sub(self.start).unwrap();
            Choice::print_choice_loop_max_items(iter, config, handle, range);
        }
    }

    fn print_choice_loop_max_items<W, T, I>(
        iter: I,
        config: &Config,
        handle: &mut W,
        max_items: isize,
    ) where
        W: WriteReceiver,
        T: Writeable,
        I: Iterator<Item = T>,
    {
        let mut peek_iter = iter.peekable();
        for i in 0..=max_items {
            match peek_iter.next() {
                Some(s) => {
                    handle.write_choice(s, config, peek_iter.peek().is_some() && i != max_items);
                }
                None => break,
            };
        }
    }

    fn print_choice_negative<W, T, I>(&self, iter: I, config: &Config, handle: &mut W)
    where
        W: WriteReceiver,
        T: Writeable,
        I: Iterator<Item = T>,
    {
        let vec = Vec::from_iter(iter);
        let (start, end) = self.get_negative_start_end(&vec);

        if end > start {
            for word in vec[start..std::cmp::min(end, vec.len() - 1)].iter() {
                handle.write_choice(*word, config, true);
            }
            handle.write_choice(vec[std::cmp::min(end, vec.len() - 1)], config, false);
        } else if self.start < 0 {
            for word in vec[end + 1..=std::cmp::min(start, vec.len() - 1)]
                .iter()
                .rev()
            {
                handle.write_choice(*word, config, true);
            }
            handle.write_choice(vec[end], config, false);
        }
    }

    fn print_choice_reverse<W, T, I>(&self, mut iter: I, config: &Config, handle: &mut W)
    where
        W: WriteReceiver,
        T: Writeable,
        I: Iterator<Item = T>,
    {
        if self.end > 0 {
            iter.nth((self.end - 1).try_into().unwrap());
        }

        let mut stack = Vec::new();
        for i in 0..=(self.start - self.end) {
            match iter.next() {
                Some(s) => stack.push(s),
                None => break,
            }

            if self.start <= self.end + i {
                break;
            }
        }

        let mut peek_iter = stack.iter().rev().peekable();
        loop {
            match peek_iter.next() {
                Some(s) => handle.write_choice(*s, config, peek_iter.peek().is_some()),
                None => break,
            }
        }
    }

    fn get_negative_start_end<T>(&self, vec: &Vec<T>) -> (usize, usize) {
        let start = if self.start >= 0 {
            self.start.try_into().unwrap()
        } else {
            vec.len()
                .checked_sub(self.start.abs().try_into().unwrap())
                .unwrap()
        };

        let end = if self.end >= 0 {
            self.end.try_into().unwrap()
        } else {
            vec.len()
                .checked_sub(self.end.abs().try_into().unwrap())
                .unwrap()
        };

        return (start, end);
    }
}

#[cfg(test)]
mod tests {

    use crate::config::Config;
    use crate::opt::Opt;
    use std::ffi::OsString;
    use std::io::{self, BufWriter, Write};
    use structopt::StructOpt;

    impl Config {
        pub fn from_iter<I>(iter: I) -> Self
        where
            I: IntoIterator,
            I::Item: Into<OsString> + Clone,
        {
            return Config::new(Opt::from_iter(iter));
        }
    }

    struct MockStdout {
        pub buffer: String,
    }

    impl MockStdout {
        fn new() -> Self {
            MockStdout {
                buffer: String::new(),
            }
        }

        fn str_from_buf_writer(b: BufWriter<MockStdout>) -> String {
            match b.into_inner() {
                Ok(b) => b.buffer,
                Err(_) => panic!("Failed to access BufWriter inner writer"),
            }
            .trim_end()
            .to_string()
        }
    }

    impl Write for MockStdout {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            let mut bytes_written = 0;
            for i in buf {
                self.buffer.push(*i as char);
                bytes_written += 1;
            }
            Ok(bytes_written)
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    mod print_choice_tests {
        use super::*;

        fn test_fn(vec: Vec<&str>, input: &str, output: &str) {
            let config = Config::from_iter(vec);
            let mut handle = BufWriter::new(MockStdout::new());

            config.opt.choices[0].print_choice(&String::from(input), &config, &mut handle);

            assert_eq!(
                String::from(output),
                MockStdout::str_from_buf_writer(handle)
            );
        }

        #[test]
        fn print_0() {
            test_fn(vec!["choose", "0"], "rust is pretty cool", "rust");
        }

        #[test]
        fn print_after_end() {
            test_fn(vec!["choose", "10"], "rust is pretty cool", "");
        }

        #[test]
        fn print_out_of_order() {
            let config = Config::from_iter(vec!["choose", "3", "1"]);
            let mut handle = BufWriter::new(MockStdout::new());
            let mut handle1 = BufWriter::new(MockStdout::new());

            config.opt.choices[0].print_choice(
                &String::from("rust is pretty cool"),
                &config,
                &mut handle,
            );

            assert_eq!(
                String::from("cool"),
                MockStdout::str_from_buf_writer(handle)
            );

            config.opt.choices[1].print_choice(
                &String::from("rust is pretty cool"),
                &config,
                &mut handle1,
            );

            assert_eq!(String::from("is"), MockStdout::str_from_buf_writer(handle1));
        }

        #[test]
        fn print_1_to_3_exclusive() {
            test_fn(
                vec!["choose", "1:3", "-x"],
                "rust is pretty cool",
                "is pretty",
            );
        }

        #[test]
        fn print_1_to_3() {
            test_fn(
                vec!["choose", "1:3"],
                "rust is pretty cool",
                "is pretty cool",
            );
        }

        #[test]
        fn print_1_to_3_separated_by_hashtag() {
            test_fn(
                vec!["choose", "1:3", "-f", "#"],
                "rust#is#pretty#cool",
                "is pretty cool",
            );
        }

        #[test]
        fn print_1_to_3_separated_by_varying_multiple_hashtag_exclusive() {
            test_fn(
                vec!["choose", "1:3", "-f", "#", "-x"],
                "rust##is###pretty####cool",
                "is pretty",
            );
        }

        #[test]
        fn print_1_to_3_separated_by_varying_multiple_hashtag() {
            test_fn(
                vec!["choose", "1:3", "-f", "#"],
                "rust##is###pretty####cool",
                "is pretty cool",
            );
        }

        #[test]
        fn print_1_to_3_separated_by_regex_group_vowels_exclusive() {
            test_fn(
                vec!["choose", "1:3", "-f", "[aeiou]", "-x"],
                "the quick brown fox jumped over the lazy dog",
                " q ck br",
            );
        }

        #[test]
        fn print_1_to_3_separated_by_regex_group_vowels() {
            test_fn(
                vec!["choose", "1:3", "-f", "[aeiou]"],
                "the quick brown fox jumped over the lazy dog",
                " q ck br wn f",
            );
        }

        #[test]
        fn print_3_to_1() {
            test_fn(
                vec!["choose", "3:1"],
                "rust lang is pretty darn cool",
                "pretty is lang",
            );
        }

        #[test]
        fn print_3_to_1_exclusive() {
            test_fn(
                vec!["choose", "3:1", "-x"],
                "rust lang is pretty darn cool",
                "is lang",
            );
        }

        #[test]
        fn print_1_to_3_nonexistant_field_separator() {
            test_fn(
                vec!["choose", "1:3", "-f", "#"],
                "rust lang is pretty darn cool",
                "",
            );
        }

        #[test]
        fn print_0_nonexistant_field_separator() {
            test_fn(
                vec!["choose", "0", "-f", "#"],
                "rust lang is pretty darn cool",
                "rust lang is pretty darn cool",
            );
        }

        #[test]
        fn print_0_to_3_nonexistant_field_separator() {
            test_fn(
                vec!["choose", "0:3", "-f", "#"],
                "rust lang is pretty darn cool",
                "rust lang is pretty darn cool",
            );
        }

        #[test]
        fn print_0_with_preceding_separator() {
            test_fn(
                vec!["choose", "0"],
                "   rust lang is pretty darn cool",
                "rust",
            );
        }

        #[test]
        fn print_neg3_to_neg1() {
            test_fn(
                vec!["choose", "-3:-1"],
                "rust lang is pretty darn cool",
                "pretty darn cool",
            );
        }

        #[test]
        fn print_neg1_to_neg3() {
            test_fn(
                vec!["choose", "-1:-3"],
                "rust lang is pretty darn cool",
                "cool darn pretty",
            );
        }

        #[test]
        fn print_neg2_to_end() {
            test_fn(
                vec!["choose", "-2:"],
                "rust lang is pretty darn cool",
                "darn cool",
            );
        }

        #[test]
        fn print_start_to_neg3() {
            test_fn(
                vec!["choose", ":-3"],
                "rust lang is pretty darn cool",
                "rust lang is pretty",
            );
        }

        #[test]
        fn print_1_to_neg3() {
            test_fn(
                vec!["choose", "1:-3"],
                "rust lang is pretty darn cool",
                "lang is pretty",
            );
        }

        #[test]
        fn print_5_to_neg3_empty() {
            test_fn(vec!["choose", "5:-3"], "rust lang is pretty darn cool", "");
        }

        #[test]
        fn print_0_to_2_greedy() {
            test_fn(vec!["choose", "0:2", "-f", ":"], "a:b::c:::d", "a b c");
        }

        #[test]
        fn print_0_to_2_non_greedy() {
            test_fn(vec!["choose", "0:2", "-n", "-f", ":"], "a:b::c:::d", "a b");
        }

        #[test]
        fn print_2_to_neg_1_non_greedy_negative() {
            test_fn(vec!["choose", "2:-1", "-n", "-f", ":"], "a:b::c:::d", "c d");
        }

        #[test]
        fn print_2_to_0_non_greedy_reversed() {
            test_fn(vec!["choose", "2:0", "-n", "-f", ":"], "a:b::c:::d", "b a");
        }

        #[test]
        fn print_neg_1_to_neg_3_non_greedy_negative_reversed() {
            test_fn(vec!["choose", "-1:-3", "-n", "-f", ":"], "a:b::c:::d", "d");
        }

        #[test]
        fn print_1_to_3_with_output_field_separator() {
            test_fn(vec!["choose", "1:3", "-o", "#"], "a b c d", "b#c#d");
        }

        #[test]
        fn print_1_and_3_with_output_field_separator() {
            test_fn(vec!["choose", "1", "3", "-o", "#"], "a b c d", "b");
        }

        #[test]
        fn print_2_to_4_with_output_field_separator() {
            test_fn(
                vec!["choose", "2:4", "-o", "%"],
                "Lorem ipsum dolor sit amet, consectetur",
                "dolor%sit%amet,",
            );
        }

        #[test]
        fn print_3_to_1_with_output_field_separator() {
            test_fn(vec!["choose", "3:1", "-o", "#"], "a b c d", "d#c#b");
        }

        #[test]
        fn print_0_to_neg_2_with_output_field_separator() {
            test_fn(vec!["choose", "0:-2", "-o", "#"], "a b c d", "a#b#c");
        }

        #[test]
        fn print_0_to_2_with_empty_output_field_separator() {
            test_fn(vec!["choose", "0:2", "-o", ""], "a b c d", "abc");
        }

        #[test]
        fn print_0_to_2_character_wise() {
            test_fn(vec!["choose", "0:2", "-c"], "abcd\n", "abc");
        }

        #[test]
        fn print_2_to_end_character_wise() {
            test_fn(vec!["choose", "2:", "-c"], "abcd\n", "cd");
        }

        #[test]
        fn print_start_to_2_character_wise() {
            test_fn(vec!["choose", ":2", "-c"], "abcd\n", "abc");
        }

        #[test]
        fn print_0_to_2_character_wise_exclusive() {
            test_fn(vec!["choose", "0:2", "-c", "-x"], "abcd\n", "ab");
        }

        #[test]
        fn print_0_to_2_character_wise_with_output_delimeter() {
            test_fn(vec!["choose", "0:2", "-c", "-o", ":"], "abcd\n", "a:b:c");
        }

        #[test]
        fn print_after_end_character_wise() {
            test_fn(vec!["choose", "0:9", "-c"], "abcd\n", "abcd");
        }

        #[test]
        fn print_2_to_0_character_wise() {
            test_fn(vec!["choose", "2:0", "-c"], "abcd\n", "cba");
        }

        #[test]
        fn print_neg_2_to_end_character_wise() {
            test_fn(vec!["choose", "-2:", "-c"], "abcd\n", "cd");
        }

        #[test]
        fn print_1_to_3_exclusive_rust_syntax_inclusive() {
            test_fn(
                vec!["choose", "1..=3", "-x"],
                "rust is pretty cool",
                "is pretty cool",
            );
        }

        #[test]
        fn print_1_to_3_rust_syntax_inclusive() {
            test_fn(
                vec!["choose", "1..=3"],
                "rust is pretty cool",
                "is pretty cool",
            );
        }

        #[test]
        fn print_1_to_3_separated_by_hashtag_rust_syntax_inclusive() {
            test_fn(
                vec!["choose", "1..=3", "-f", "#"],
                "rust#is#pretty#cool",
                "is pretty cool",
            );
        }

        #[test]
        fn print_1_to_3_separated_by_varying_multiple_hashtag_exclusive_rust_syntax_inclusive() {
            test_fn(
                vec!["choose", "1..=3", "-f", "#", "-x"],
                "rust##is###pretty####cool",
                "is pretty cool",
            );
        }

        #[test]
        fn print_1_to_3_separated_by_varying_multiple_hashtag_rust_syntax_inclusive() {
            test_fn(
                vec!["choose", "1..=3", "-f", "#"],
                "rust##is###pretty####cool",
                "is pretty cool",
            );
        }

        #[test]
        fn print_1_to_3_separated_by_regex_group_vowels_exclusive_rust_syntax_inclusive() {
            test_fn(
                vec!["choose", "1..=3", "-f", "[aeiou]", "-x"],
                "the quick brown fox jumped over the lazy dog",
                " q ck br wn f",
            );
        }

        #[test]
        fn print_1_to_3_separated_by_regex_group_vowels_rust_syntax_inclusive() {
            test_fn(
                vec!["choose", "1..=3", "-f", "[aeiou]"],
                "the quick brown fox jumped over the lazy dog",
                " q ck br wn f",
            );
        }

        #[test]
        fn print_3_to_1_rust_syntax_inclusive() {
            test_fn(
                vec!["choose", "3..=1"],
                "rust lang is pretty darn cool",
                "pretty is lang",
            );
        }

        #[test]
        fn print_3_to_1_exclusive_rust_syntax_inclusive() {
            test_fn(
                vec!["choose", "3..=1", "-x"],
                "rust lang is pretty darn cool",
                "pretty is lang",
            );
        }

        #[test]
        fn print_1_to_3_nonexistant_field_separator_rust_syntax_inclusive() {
            test_fn(
                vec!["choose", "1..=3", "-f", "#"],
                "rust lang is pretty darn cool",
                "",
            );
        }

        #[test]
        fn print_0_to_3_nonexistant_field_separator_rust_syntax_inclusive() {
            test_fn(
                vec!["choose", "0..=3", "-f", "#"],
                "rust lang is pretty darn cool",
                "rust lang is pretty darn cool",
            );
        }

        #[test]
        fn print_neg1_to_neg1_rust_syntax_inclusive() {
            test_fn(
                vec!["choose", "-3..=-1"],
                "rust lang is pretty darn cool",
                "pretty darn cool",
            );
        }

        #[test]
        fn print_neg1_to_neg3_rust_syntax_inclusive() {
            test_fn(
                vec!["choose", "-1..=-3"],
                "rust lang is pretty darn cool",
                "cool darn pretty",
            );
        }

        #[test]
        fn print_neg2_to_end_rust_syntax_inclusive() {
            test_fn(
                vec!["choose", "-2..="],
                "rust lang is pretty darn cool",
                "darn cool",
            );
        }

        #[test]
        fn print_start_to_neg3_rust_syntax_inclusive() {
            test_fn(
                vec!["choose", "..=-3"],
                "rust lang is pretty darn cool",
                "rust lang is pretty",
            );
        }

        #[test]
        fn print_1_to_neg3_rust_syntax_inclusive() {
            test_fn(
                vec!["choose", "1..=-3"],
                "rust lang is pretty darn cool",
                "lang is pretty",
            );
        }

        #[test]
        fn print_5_to_neg3_empty_rust_syntax_inclusive() {
            test_fn(
                vec!["choose", "5..=-3"],
                "rust lang is pretty darn cool",
                "",
            );
        }

        #[test]
        fn print_0_to_2_greedy_rust_syntax_inclusive() {
            test_fn(vec!["choose", "0..=2", "-f", ":"], "a:b::c:::d", "a b c");
        }

        #[test]
        fn print_0_to_2_non_greedy_rust_syntax_inclusive() {
            test_fn(
                vec!["choose", "0..=2", "-n", "-f", ":"],
                "a:b::c:::d",
                "a b",
            );
        }

        #[test]
        fn print_2_to_neg_1_non_greedy_negative_rust_syntax_inclusive() {
            test_fn(
                vec!["choose", "2..=-1", "-n", "-f", ":"],
                "a:b::c:::d",
                "c d",
            );
        }

        #[test]
        fn print_2_to_0_non_greedy_reversed_rust_syntax_inclusive() {
            test_fn(
                vec!["choose", "2..=0", "-n", "-f", ":"],
                "a:b::c:::d",
                "b a",
            );
        }

        #[test]
        fn print_neg_1_to_neg_3_non_greedy_negative_reversed_rust_syntax_inclusive() {
            test_fn(
                vec!["choose", "-1..=-3", "-n", "-f", ":"],
                "a:b::c:::d",
                "d",
            );
        }

        #[test]
        fn print_1_to_3_with_output_field_separator_rust_syntax_inclusive() {
            test_fn(vec!["choose", "1..=3", "-o", "#"], "a b c d", "b#c#d");
        }

        #[test]
        fn print_1_and_3_with_output_field_separator_rust_syntax_inclusive() {
            let config = Config::from_iter(vec!["choose", "1", "3", "-o", "#"]);
            let mut handle = BufWriter::new(MockStdout::new());
            config.opt.choices[0].print_choice(&String::from("a b c d"), &config, &mut handle);
            handle.write(&config.output_separator).unwrap();
            config.opt.choices[1].print_choice(&String::from("a b c d"), &config, &mut handle);
            assert_eq!(String::from("b#d"), MockStdout::str_from_buf_writer(handle));
        }

        #[test]
        fn print_2_to_4_with_output_field_separator_rust_syntax_inclusive() {
            test_fn(
                vec!["choose", "2..=4", "-o", "%"],
                "Lorem ipsum dolor sit amet, consectetur",
                "dolor%sit%amet,",
            );
        }

        #[test]
        fn print_3_to_1_with_output_field_separator_rust_syntax_inclusive() {
            test_fn(vec!["choose", "3..=1", "-o", "#"], "a b c d", "d#c#b");
        }

        #[test]
        fn print_0_to_neg_2_with_output_field_separator_rust_syntax_inclusive() {
            test_fn(vec!["choose", "0..=-2", "-o", "#"], "a b c d", "a#b#c");
        }

        #[test]
        fn print_0_to_2_with_empty_output_field_separator_rust_syntax_inclusive() {
            test_fn(vec!["choose", "0..=2", "-o", ""], "a b c d", "abc");
        }

        #[test]
        fn print_0_to_2_character_wise_rust_syntax_inclusive() {
            test_fn(vec!["choose", "0..=2", "-c"], "abcd\n", "abc");
        }

        #[test]
        fn print_2_to_end_character_wise_rust_syntax_inclusive() {
            test_fn(vec!["choose", "2..=", "-c"], "abcd\n", "cd");
        }

        #[test]
        fn print_start_to_2_character_wise_rust_syntax_inclusive() {
            test_fn(vec!["choose", "..=2", "-c"], "abcd\n", "abc");
        }

        #[test]
        fn print_0_to_2_character_wise_exclusive_rust_syntax_inclusive() {
            test_fn(vec!["choose", "0..=2", "-c", "-x"], "abcd\n", "abc");
        }

        #[test]
        fn print_0_to_2_character_wise_with_output_delimeter_rust_syntax_inclusive() {
            test_fn(vec!["choose", "0..=2", "-c", "-o", ":"], "abcd\n", "a:b:c");
        }

        #[test]
        fn print_after_end_character_wise_rust_syntax_inclusive() {
            test_fn(vec!["choose", "0..=9", "-c"], "abcd\n", "abcd");
        }

        #[test]
        fn print_2_to_0_character_wise_rust_syntax_inclusive() {
            test_fn(vec!["choose", "2..=0", "-c"], "abcd\n", "cba");
        }

        #[test]
        fn print_neg_2_to_end_character_wise_rust_syntax_inclusive() {
            test_fn(vec!["choose", "-2..=", "-c"], "abcd\n", "cd");
        }

        #[test]
        fn print_1_to_3_exclusive_rust_syntax_exclusive() {
            test_fn(
                vec!["choose", "1..3", "-x"],
                "rust is pretty cool",
                "is pretty",
            );
        }

        #[test]
        fn print_1_to_3_rust_syntax_exclusive() {
            test_fn(vec!["choose", "1..3"], "rust is pretty cool", "is pretty");
        }

        #[test]
        fn print_1_to_3_separated_by_hashtag_rust_syntax_exclusive() {
            test_fn(
                vec!["choose", "1..3", "-f", "#"],
                "rust#is#pretty#cool",
                "is pretty",
            );
        }

        #[test]
        fn print_1_to_3_separated_by_varying_multiple_hashtag_exclusive_rust_syntax_exclusive() {
            test_fn(
                vec!["choose", "1..3", "-f", "#", "-x"],
                "rust##is###pretty####cool",
                "is pretty",
            );
        }

        #[test]
        fn print_1_to_3_separated_by_varying_multiple_hashtag_rust_syntax_exclusive() {
            test_fn(
                vec!["choose", "1..3", "-f", "#"],
                "rust##is###pretty####cool",
                "is pretty",
            );
        }

        #[test]
        fn print_1_to_3_separated_by_regex_group_vowels_exclusive_rust_syntax_exclusive() {
            test_fn(
                vec!["choose", "1..3", "-f", "[aeiou]", "-x"],
                "the quick brown fox jumped over the lazy dog",
                " q ck br",
            );
        }

        #[test]
        fn print_1_to_3_separated_by_regex_group_vowels_rust_syntax_exclusive() {
            test_fn(
                vec!["choose", "1..3", "-f", "[aeiou]"],
                "the quick brown fox jumped over the lazy dog",
                " q ck br",
            );
        }

        #[test]
        fn print_3_to_1_rust_syntax_exclusive() {
            test_fn(
                vec!["choose", "3..1"],
                "rust lang is pretty darn cool",
                "is lang",
            );
        }

        #[test]
        fn print_3_to_1_exclusive_rust_syntax_exclusive() {
            test_fn(
                vec!["choose", "3..1", "-x"],
                "rust lang is pretty darn cool",
                "is lang",
            );
        }

        #[test]
        fn print_1_to_3_nonexistant_field_separator_rust_syntax_exclusive() {
            test_fn(
                vec!["choose", "1..3", "-f", "#"],
                "rust lang is pretty darn cool",
                "",
            );
        }

        #[test]
        fn print_0_to_3_nonexistant_field_separator_rust_syntax_exclusive() {
            test_fn(
                vec!["choose", "0..3", "-f", "#"],
                "rust lang is pretty darn cool",
                "rust lang is pretty darn cool",
            );
        }

        #[test]
        fn print_neg3_to_neg1_rust_syntax_exclusive() {
            test_fn(
                vec!["choose", "-3..-1"],
                "rust lang is pretty darn cool",
                "pretty darn",
            );
        }

        #[test]
        fn print_neg1_to_neg3_rust_syntax_exclusive() {
            test_fn(
                vec!["choose", "-1..-3"],
                "rust lang is pretty darn cool",
                "darn pretty",
            );
        }

        #[test]
        fn print_neg2_to_end_rust_syntax_exclusive() {
            test_fn(
                vec!["choose", "-2.."],
                "rust lang is pretty darn cool",
                "darn cool",
            );
        }

        #[test]
        fn print_start_to_neg3_rust_syntax_exclusive() {
            test_fn(
                vec!["choose", "..-3"],
                "rust lang is pretty darn cool",
                "rust lang is",
            );
        }

        #[test]
        fn print_1_to_neg3_rust_syntax_exclusive() {
            test_fn(
                vec!["choose", "1..-3"],
                "rust lang is pretty darn cool",
                "lang is",
            );
        }

        #[test]
        fn print_5_to_neg3_empty_rust_syntax_exclusive() {
            test_fn(vec!["choose", "5..-3"], "rust lang is pretty darn cool", "");
        }

        #[test]
        fn print_0_to_2_greedy_rust_syntax_exclusive() {
            test_fn(vec!["choose", "0..2", "-f", ":"], "a:b::c:::d", "a b");
        }

        #[test]
        fn print_0_to_2_non_greedy_rust_syntax_exclusive() {
            test_fn(vec!["choose", "0..2", "-n", "-f", ":"], "a:b::c:::d", "a b");
        }

        #[test]
        fn print_2_to_neg_1_non_greedy_negative_rust_syntax_exclusive() {
            test_fn(vec!["choose", "2..-1", "-n", "-f", ":"], "a:b::c:::d", "c");
        }

        #[test]
        fn print_2_to_0_non_greedy_reversed_rust_syntax_exclusive() {
            test_fn(vec!["choose", "2..0", "-n", "-f", ":"], "a:b::c:::d", "b a");
        }

        #[test]
        fn print_neg_1_to_neg_3_non_greedy_negative_reversed_rust_syntax_exclusive() {
            test_fn(vec!["choose", "-1..-3", "-n", "-f", ":"], "a:b::c:::d", "");
        }

        #[test]
        fn print_1_to_3_with_output_field_separator_rust_syntax_exclusive() {
            test_fn(vec!["choose", "1..3", "-o", "#"], "a b c d", "b#c");
        }

        #[test]
        fn print_2_to_4_with_output_field_separator_rust_syntax_exclusive() {
            test_fn(
                vec!["choose", "2..4", "-o", "%"],
                "Lorem ipsum dolor sit amet, consectetur",
                "dolor%sit",
            );
        }

        #[test]
        fn print_3_to_1_with_output_field_separator_rust_syntax_exclusive() {
            test_fn(vec!["choose", "3..1", "-o", "#"], "a b c d", "c#b");
        }

        #[test]
        fn print_0_to_neg_2_with_output_field_separator_rust_syntax_exclusive() {
            test_fn(vec!["choose", "0..-2", "-o", "#"], "a b c d", "a#b");
        }

        #[test]
        fn print_0_to_2_with_empty_output_field_separator_rust_syntax_exclusive() {
            test_fn(vec!["choose", "0..2", "-o", ""], "a b c d", "ab");
        }

        #[test]
        fn print_0_to_2_character_wise_rust_syntax_exclusive() {
            test_fn(vec!["choose", "0..2", "-c"], "abcd\n", "ab");
        }

        #[test]
        fn print_2_to_end_character_wise_rust_syntax_exclusive() {
            test_fn(vec!["choose", "2..", "-c"], "abcd\n", "cd");
        }

        #[test]
        fn print_start_to_2_character_wise_rust_syntax_exclusive() {
            test_fn(vec!["choose", "..2", "-c"], "abcd\n", "ab");
        }

        #[test]
        fn print_0_to_2_character_wise_exclusive_rust_syntax_exclusive() {
            test_fn(vec!["choose", "0..2", "-c", "-x"], "abcd\n", "ab");
        }

        #[test]
        fn print_0_to_2_character_wise_with_output_delimeter_rust_syntax_exclusive() {
            test_fn(vec!["choose", "0..2", "-c", "-o", ":"], "abcd\n", "a:b");
        }

        #[test]
        fn print_after_end_character_wise_rust_syntax_exclusive() {
            test_fn(vec!["choose", "0..9", "-c"], "abcd\n", "abcd");
        }

        #[test]
        fn print_2_to_0_character_wise_rust_syntax_exclusive() {
            test_fn(vec!["choose", "2..0", "-c"], "abcd\n", "ba");
        }

        #[test]
        fn print_neg_2_to_end_character_wise_rust_syntax_exclusive() {
            test_fn(vec!["choose", "-2..", "-c"], "abcd\n", "cd");
        }

        #[test]
        fn print_2_exclusive() {
            test_fn(vec!["choose", "2", "-x"], "a b c d", "c");
        }

        #[test]
        fn print_2_one_indexed() {
            test_fn(vec!["choose", "2", "--one-indexed"], "a b c d", "b");
        }

        #[test]
        fn print_2_to_4_one_indexed() {
            test_fn(vec!["choose", "2:4", "--one-indexed"], "a b c d", "b c d");
        }

        #[test]
        fn print_2_to_end_one_indexed() {
            test_fn(vec!["choose", "2:", "--one-indexed"], "a b c d", "b c d");
        }

        #[test]
        fn print_start_to_2_one_indexed() {
            test_fn(vec!["choose", ":2", "--one-indexed"], "a b c d", "a b");
        }

        #[test]
        fn print_2_to_4_one_indexed_exclusive() {
            test_fn(
                vec!["choose", "2:4", "--one-indexed", "-x"],
                "a b c d",
                "b c",
            );
        }

        #[test]
        fn print_4_to_2_one_indexed() {
            test_fn(vec!["choose", "4:2", "--one-indexed"], "a b c d", "d c b");
        }

        #[test]
        fn print_neg_4_to_2_one_indexed() {
            test_fn(vec!["choose", "-4:2", "--one-indexed"], "a b c d", "a b");
        }

        #[test]
        fn print_2_to_4_newline_ofs() {
            test_fn(
                vec!["choose", "2:4", "-o", r#"\n"#],
                "a b c d e f",
                "c\nd\ne",
            );
        }
    }

    mod is_reverse_range_tests {
        use super::*;

        #[test]
        fn is_field_reversed() {
            let config = Config::from_iter(vec!["choose", "0"]);
            assert_eq!(false, config.opt.choices[0].is_reverse_range());
        }

        #[test]
        fn is_field_range_no_start_reversed() {
            let config = Config::from_iter(vec!["choose", ":2"]);
            assert_eq!(false, config.opt.choices[0].is_reverse_range());
        }

        #[test]
        fn is_field_range_no_end_reversed() {
            let config = Config::from_iter(vec!["choose", "2:"]);
            assert_eq!(false, config.opt.choices[0].is_reverse_range());
        }

        #[test]
        fn is_field_range_no_start_or_end_reversed() {
            let config = Config::from_iter(vec!["choose", ":"]);
            assert_eq!(false, config.opt.choices[0].is_reverse_range());
        }

        #[test]
        fn is_reversed_field_range_reversed() {
            let config = Config::from_iter(vec!["choose", "4:2"]);
            assert_eq!(true, config.opt.choices[0].is_reverse_range());
        }
    }
}
