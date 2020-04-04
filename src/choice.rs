use std::convert::TryInto;

use crate::config::Config;
use crate::io::{BufWriter, Write};

#[derive(Debug)]
pub struct Choice {
    pub start: isize,
    pub end: isize,
    negative_index: bool,
    reversed: bool,
}

impl Choice {
    pub fn new(start: isize, end: isize) -> Self {
        let negative_index = start < 0 || end < 0;
        let reversed = end < start;
        Choice {
            start,
            end,
            negative_index,
            reversed,
        }
    }

    pub fn print_choice<WriterType: Write>(
        &self,
        line: &String,
        config: &Config,
        handle: &mut BufWriter<WriterType>,
    ) {
        let mut line_iter = config.separator.split(line).filter(|s| !s.is_empty());

        if self.is_reverse_range() && !self.has_negative_index() {
            if self.end > 0 {
                line_iter.nth((self.end - 1).try_into().unwrap());
            }

            let mut stack = Vec::new();
            for i in 0..=(self.start - self.end) {
                match line_iter.next() {
                    Some(s) => stack.push(s),
                    None => break,
                }

                if self.start <= self.end + i {
                    break;
                }
            }

            loop {
                match stack.pop() {
                    Some(s) => Choice::write_bytes(handle, s.as_bytes()),
                    None => break,
                }
            }
        } else if self.has_negative_index() {
            let vec = line_iter.collect::<Vec<&str>>();

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

            if end > start {
                for word in vec[start..=std::cmp::min(end, vec.len() - 1)].iter() {
                    Choice::write_bytes(handle, word.as_bytes());
                }
            } else if self.start < 0 {
                for word in vec[end..=std::cmp::min(start, vec.len() - 1)].iter().rev() {
                    Choice::write_bytes(handle, word.as_bytes());
                }
            }
        } else {
            if self.start > 0 {
                line_iter.nth((self.start - 1).try_into().unwrap());
            }

            for i in 0..=(self.end - self.start) {
                match line_iter.next() {
                    Some(s) => Choice::write_bytes(handle, s.as_bytes()),
                    None => break,
                };

                if self.end <= self.start + i {
                    break;
                }
            }
        }
    }

    fn write_bytes<WriterType: Write>(handle: &mut BufWriter<WriterType>, b: &[u8]) {
        match handle.write(b) {
            Ok(_) => (),
            Err(e) => eprintln!("Failed to write to output: {}", e),
        }
        match handle.write(b" ") {
            Ok(_) => (),
            Err(e) => eprintln!("Failed to write to output: {}", e),
        }
    }

    pub fn is_reverse_range(&self) -> bool {
        self.reversed
    }

    pub fn has_negative_index(&self) -> bool {
        self.negative_index
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

        #[test]
        fn print_0() {
            let config = Config::from_iter(vec!["choose", "0"]);
            let mut handle = BufWriter::new(MockStdout::new());

            config.opt.choice[0].print_choice(
                &String::from("rust is pretty cool"),
                &config,
                &mut handle,
            );

            assert_eq!(
                String::from("rust"),
                MockStdout::str_from_buf_writer(handle)
            );
        }

        #[test]
        fn print_after_end() {
            let config = Config::from_iter(vec!["choose", "10"]);
            let mut handle = BufWriter::new(MockStdout::new());

            config.opt.choice[0].print_choice(
                &String::from("rust is pretty cool"),
                &config,
                &mut handle,
            );

            assert_eq!(String::new(), MockStdout::str_from_buf_writer(handle));
        }

        #[test]
        fn print_out_of_order() {
            let config = Config::from_iter(vec!["choose", "3", "1"]);
            let mut handle = BufWriter::new(MockStdout::new());
            let mut handle1 = BufWriter::new(MockStdout::new());

            config.opt.choice[0].print_choice(
                &String::from("rust is pretty cool"),
                &config,
                &mut handle,
            );

            assert_eq!(
                String::from("cool"),
                MockStdout::str_from_buf_writer(handle)
            );

            config.opt.choice[1].print_choice(
                &String::from("rust is pretty cool"),
                &config,
                &mut handle1,
            );

            assert_eq!(String::from("is"), MockStdout::str_from_buf_writer(handle1));
        }

        #[test]
        fn print_1_to_3_exclusive() {
            let config = Config::from_iter(vec!["choose", "1:3", "-x"]);
            let mut handle = BufWriter::new(MockStdout::new());
            config.opt.choice[0].print_choice(
                &String::from("rust is pretty cool"),
                &config,
                &mut handle,
            );
            assert_eq!(
                String::from("is pretty"),
                MockStdout::str_from_buf_writer(handle)
            );
        }

        #[test]
        fn print_1_to_3() {
            let config = Config::from_iter(vec!["choose", "1:3"]);
            let mut handle = BufWriter::new(MockStdout::new());
            config.opt.choice[0].print_choice(
                &String::from("rust is pretty cool"),
                &config,
                &mut handle,
            );
            assert_eq!(
                String::from("is pretty cool"),
                MockStdout::str_from_buf_writer(handle)
            );
        }

        #[test]
        fn print_1_to_3_separated_by_hashtag() {
            let config = Config::from_iter(vec!["choose", "1:3", "-f", "#"]);
            let mut handle = BufWriter::new(MockStdout::new());
            config.opt.choice[0].print_choice(
                &String::from("rust#is#pretty#cool"),
                &config,
                &mut handle,
            );
            assert_eq!(
                String::from("is pretty cool"),
                MockStdout::str_from_buf_writer(handle)
            );
        }

        #[test]
        fn print_1_to_3_separated_by_varying_multiple_hashtag_exclusive() {
            let config = Config::from_iter(vec!["choose", "1:3", "-f", "#", "-x"]);
            let mut handle = BufWriter::new(MockStdout::new());
            config.opt.choice[0].print_choice(
                &String::from("rust##is###pretty####cool"),
                &config,
                &mut handle,
            );
            assert_eq!(
                String::from("is pretty"),
                MockStdout::str_from_buf_writer(handle)
            );
        }

        #[test]
        fn print_1_to_3_separated_by_varying_multiple_hashtag() {
            let config = Config::from_iter(vec!["choose", "1:3", "-f", "#"]);
            let mut handle = BufWriter::new(MockStdout::new());
            config.opt.choice[0].print_choice(
                &String::from("rust##is###pretty####cool"),
                &config,
                &mut handle,
            );
            assert_eq!(
                String::from("is pretty cool"),
                MockStdout::str_from_buf_writer(handle)
            );
        }

        #[test]
        fn print_1_to_3_separated_by_regex_group_vowels_exclusive() {
            let config = Config::from_iter(vec!["choose", "1:3", "-f", "[aeiou]", "-x"]);
            let mut handle = BufWriter::new(MockStdout::new());
            config.opt.choice[0].print_choice(
                &String::from("the quick brown fox jumped over the lazy dog"),
                &config,
                &mut handle,
            );
            assert_eq!(
                String::from(" q ck br"),
                MockStdout::str_from_buf_writer(handle)
            );
        }

        #[test]
        fn print_1_to_3_separated_by_regex_group_vowels() {
            let config = Config::from_iter(vec!["choose", "1:3", "-f", "[aeiou]"]);
            let mut handle = BufWriter::new(MockStdout::new());
            config.opt.choice[0].print_choice(
                &String::from("the quick brown fox jumped over the lazy dog"),
                &config,
                &mut handle,
            );
            assert_eq!(
                String::from(" q ck br wn f"),
                MockStdout::str_from_buf_writer(handle)
            );
        }

        #[test]
        fn print_3_to_1() {
            let config = Config::from_iter(vec!["choose", "3:1"]);
            let mut handle = BufWriter::new(MockStdout::new());
            config.opt.choice[0].print_choice(
                &String::from("rust lang is pretty darn cool"),
                &config,
                &mut handle,
            );
            assert_eq!(
                String::from("pretty is lang"),
                MockStdout::str_from_buf_writer(handle)
            );
        }

        #[test]
        fn print_3_to_1_exclusive() {
            let config = Config::from_iter(vec!["choose", "3:1", "-x"]);
            let mut handle = BufWriter::new(MockStdout::new());
            config.opt.choice[0].print_choice(
                &String::from("rust lang is pretty darn cool"),
                &config,
                &mut handle,
            );
            assert_eq!(
                String::from("is lang"),
                MockStdout::str_from_buf_writer(handle)
            );
        }

        #[test]
        fn print_1_to_3_nonexistant_field_separator() {
            let config = Config::from_iter(vec!["choose", "1:3", "-f", "#"]);
            let mut handle = BufWriter::new(MockStdout::new());
            config.opt.choice[0].print_choice(
                &String::from("rust lang is pretty darn cool"),
                &config,
                &mut handle,
            );
            assert_eq!(String::from(""), MockStdout::str_from_buf_writer(handle));
        }

        #[test]
        fn print_0_nonexistant_field_separator() {
            let config = Config::from_iter(vec!["choose", "0", "-f", "#"]);
            let mut handle = BufWriter::new(MockStdout::new());
            config.opt.choice[0].print_choice(
                &String::from("rust lang is pretty darn cool"),
                &config,
                &mut handle,
            );
            assert_eq!(
                String::from("rust lang is pretty darn cool"),
                MockStdout::str_from_buf_writer(handle)
            );
        }

        #[test]
        fn print_0_to_3_nonexistant_field_separator() {
            let config = Config::from_iter(vec!["choose", "0:3", "-f", "#"]);
            let mut handle = BufWriter::new(MockStdout::new());
            config.opt.choice[0].print_choice(
                &String::from("rust lang is pretty darn cool"),
                &config,
                &mut handle,
            );
            assert_eq!(
                String::from("rust lang is pretty darn cool"),
                MockStdout::str_from_buf_writer(handle)
            );
        }

        #[test]
        fn print_0_with_preceding_separator() {
            let config = Config::from_iter(vec!["choose", "0"]);
            let mut handle = BufWriter::new(MockStdout::new());
            config.opt.choice[0].print_choice(
                &String::from("   rust lang is pretty darn cool"),
                &config,
                &mut handle,
            );
            assert_eq!(
                String::from("rust"),
                MockStdout::str_from_buf_writer(handle)
            );
        }

        #[test]
        fn print_neg3_to_neg1() {
            let config = Config::from_iter(vec!["choose", "-3:-1"]);
            let mut handle = BufWriter::new(MockStdout::new());
            config.opt.choice[0].print_choice(
                &String::from("rust lang is pretty darn cool"),
                &config,
                &mut handle,
            );
            assert_eq!(
                String::from("pretty darn cool"),
                MockStdout::str_from_buf_writer(handle)
            );
        }

        #[test]
        fn print_neg1_to_neg3() {
            let config = Config::from_iter(vec!["choose", "-1:-3"]);
            let mut handle = BufWriter::new(MockStdout::new());
            config.opt.choice[0].print_choice(
                &String::from("rust lang is pretty darn cool"),
                &config,
                &mut handle,
            );
            assert_eq!(
                String::from("cool darn pretty"),
                MockStdout::str_from_buf_writer(handle)
            );
        }

        #[test]
        fn print_neg2_to_end() {
            let config = Config::from_iter(vec!["choose", "-2:"]);
            let mut handle = BufWriter::new(MockStdout::new());
            config.opt.choice[0].print_choice(
                &String::from("rust lang is pretty darn cool"),
                &config,
                &mut handle,
            );
            assert_eq!(
                String::from("darn cool"),
                MockStdout::str_from_buf_writer(handle)
            );
        }

        #[test]
        fn print_start_to_neg3() {
            let config = Config::from_iter(vec!["choose", ":-3"]);
            let mut handle = BufWriter::new(MockStdout::new());
            config.opt.choice[0].print_choice(
                &String::from("rust lang is pretty darn cool"),
                &config,
                &mut handle,
            );
            assert_eq!(
                String::from("rust lang is pretty"),
                MockStdout::str_from_buf_writer(handle)
            );
        }

        #[test]
        fn print_1_to_neg3() {
            let config = Config::from_iter(vec!["choose", "1:-3"]);
            let mut handle = BufWriter::new(MockStdout::new());
            config.opt.choice[0].print_choice(
                &String::from("rust lang is pretty darn cool"),
                &config,
                &mut handle,
            );
            assert_eq!(
                String::from("lang is pretty"),
                MockStdout::str_from_buf_writer(handle)
            );
        }

        #[test]
        fn print_5_to_neg3_empty() {
            let config = Config::from_iter(vec!["choose", "5:-3"]);
            let mut handle = BufWriter::new(MockStdout::new());
            config.opt.choice[0].print_choice(
                &String::from("rust lang is pretty darn cool"),
                &config,
                &mut handle,
            );
            assert_eq!(String::from(""), MockStdout::str_from_buf_writer(handle));
        }

        #[test]
        fn print_0_to_2_greedy() {
            let config = Config::from_iter(vec!["choose", "0:2", "-f", ":"]);
            let mut handle = BufWriter::new(MockStdout::new());
            config.opt.choice[0].print_choice(&String::from("a:b::c:::d"), &config, &mut handle);
            assert_eq!(
                String::from("a b c"),
                MockStdout::str_from_buf_writer(handle)
            );
        }

        #[test]
        fn print_0_to_2_non_greedy() {
            let config = Config::from_iter(vec!["choose", "0:2", "-n", "-f", ":"]);
            let mut handle = BufWriter::new(MockStdout::new());
            config.opt.choice[0].print_choice(&String::from("a:b::c:::d"), &config, &mut handle);
            assert_eq!(String::from("a b"), MockStdout::str_from_buf_writer(handle));
        }

        #[test]
        fn print_2_to_neg_1_non_greedy_negative() {
            let config = Config::from_iter(vec!["choose", "2:-1", "-n", "-f", ":"]);
            let mut handle = BufWriter::new(MockStdout::new());
            config.opt.choice[0].print_choice(&String::from("a:b::c:::d"), &config, &mut handle);
            assert_eq!(String::from("c d"), MockStdout::str_from_buf_writer(handle));
        }

        #[test]
        fn print_2_to_0_non_greedy_reversed() {
            let config = Config::from_iter(vec!["choose", "2:0", "-n", "-f", ":"]);
            let mut handle = BufWriter::new(MockStdout::new());
            config.opt.choice[0].print_choice(&String::from("a:b::c:::d"), &config, &mut handle);
            assert_eq!(String::from("b a"), MockStdout::str_from_buf_writer(handle));
        }

        #[test]
        fn print_neg_1_to_neg_3_non_greedy_negative_reversed() {
            let config = Config::from_iter(vec!["choose", "-1:-3", "-n", "-f", ":"]);
            let mut handle = BufWriter::new(MockStdout::new());
            config.opt.choice[0].print_choice(&String::from("a:b::c:::d"), &config, &mut handle);
            assert_eq!(String::from("d"), MockStdout::str_from_buf_writer(handle));
        }
    }

    mod is_reverse_range_tests {
        use super::*;

        #[test]
        fn is_field_reversed() {
            let config = Config::from_iter(vec!["choose", "0"]);
            assert_eq!(false, config.opt.choice[0].is_reverse_range());
        }

        #[test]
        fn is_field_range_no_start_reversed() {
            let config = Config::from_iter(vec!["choose", ":2"]);
            assert_eq!(false, config.opt.choice[0].is_reverse_range());
        }

        #[test]
        fn is_field_range_no_end_reversed() {
            let config = Config::from_iter(vec!["choose", "2:"]);
            assert_eq!(false, config.opt.choice[0].is_reverse_range());
        }

        #[test]
        fn is_field_range_no_start_or_end_reversed() {
            let config = Config::from_iter(vec!["choose", ":"]);
            assert_eq!(false, config.opt.choice[0].is_reverse_range());
        }

        #[test]
        fn is_reversed_field_range_reversed() {
            let config = Config::from_iter(vec!["choose", "4:2"]);
            assert_eq!(true, config.opt.choice[0].is_reverse_range());
        }
    }
}
