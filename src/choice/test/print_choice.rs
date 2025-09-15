use crate::{process_all_choices_for_line, writer::Writer};

use super::*;

fn test_fn(vec: Vec<&str>, input: &str, output: &str) {
    let config = Config::from_iter(vec);
    let mut handle = Writer::from(BufWriter::new(MockStdout::new()));

    process_all_choices_for_line(&mut handle, &config, input).unwrap();

    assert_eq!(String::from(output), MockStdout::str_from_writer(handle));
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
    let mut handle = Writer::from(BufWriter::new(MockStdout::new()));
    let mut handle1 = Writer::from(BufWriter::new(MockStdout::new()));

    config.opt.choices[0]
        .print_choice(&String::from("rust is pretty cool"), &config, &mut handle)
        .unwrap();

    assert_eq!(String::from("cool"), MockStdout::str_from_writer(handle));

    config.opt.choices[1]
        .print_choice(&String::from("rust is pretty cool"), &config, &mut handle1)
        .unwrap();

    assert_eq!(String::from("is"), MockStdout::str_from_writer(handle1));
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
    test_fn(vec!["choose", "1", "3", "-o", "#"], "a b c d", "b#d");
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
    test_fn(vec!["choose", "0:2", "-c"], "abcd", "abc");
}

#[test]
fn print_2_to_end_character_wise() {
    test_fn(vec!["choose", "2:", "-c"], "abcd", "cd");
}

#[test]
fn print_start_to_2_character_wise() {
    test_fn(vec!["choose", ":2", "-c"], "abcd", "abc");
}

#[test]
fn print_0_to_2_character_wise_exclusive() {
    test_fn(vec!["choose", "0:2", "-c", "-x"], "abcd", "ab");
}

#[test]
fn print_0_to_2_character_wise_with_output_delimeter() {
    test_fn(vec!["choose", "0:2", "-c", "-o", ":"], "abcd", "a:b:c");
}

#[test]
fn print_after_end_character_wise() {
    test_fn(vec!["choose", "0:9", "-c"], "abcd", "abcd");
}

#[test]
fn print_2_to_0_character_wise() {
    test_fn(vec!["choose", "2:0", "-c"], "abcd", "cba");
}

#[test]
fn print_neg_2_to_end_character_wise() {
    test_fn(vec!["choose", "-2:", "-c"], "abcd", "cd");
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
    test_fn(vec!["choose", "1", "3", "-o", "#"], "a b c d", "b#d");
}

#[test]
fn print_1_and_3_with_percent_output_field_separator_should_have_one_percent_sign() {
    test_fn(
        vec!["choose", "1", "3", "-o", "%"],
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor",
        "ipsum%sit",
    );
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
    test_fn(vec!["choose", "0..=2", "-c"], "abcd", "abc");
}

#[test]
fn print_2_to_end_character_wise_rust_syntax_inclusive() {
    test_fn(vec!["choose", "2..=", "-c"], "abcd", "cd");
}

#[test]
fn print_start_to_2_character_wise_rust_syntax_inclusive() {
    test_fn(vec!["choose", "..=2", "-c"], "abcd", "abc");
}

#[test]
fn print_0_to_2_character_wise_exclusive_rust_syntax_inclusive() {
    test_fn(vec!["choose", "0..=2", "-c", "-x"], "abcd", "abc");
}

#[test]
fn print_0_to_2_character_wise_with_output_delimeter_rust_syntax_inclusive() {
    test_fn(vec!["choose", "0..=2", "-c", "-o", ":"], "abcd", "a:b:c");
}

#[test]
fn print_after_end_character_wise_rust_syntax_inclusive() {
    test_fn(vec!["choose", "0..=9", "-c"], "abcd", "abcd");
}

#[test]
fn print_2_to_0_character_wise_rust_syntax_inclusive() {
    test_fn(vec!["choose", "2..=0", "-c"], "abcd", "cba");
}

#[test]
fn print_neg_2_to_end_character_wise_rust_syntax_inclusive() {
    test_fn(vec!["choose", "-2..=", "-c"], "abcd", "cd");
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
    test_fn(vec!["choose", "0..2", "-c"], "abcd", "ab");
}

#[test]
fn print_2_to_end_character_wise_rust_syntax_exclusive() {
    test_fn(vec!["choose", "2..", "-c"], "abcd", "cd");
}

#[test]
fn print_start_to_2_character_wise_rust_syntax_exclusive() {
    test_fn(vec!["choose", "..2", "-c"], "abcd", "ab");
}

#[test]
fn print_0_to_2_character_wise_exclusive_rust_syntax_exclusive() {
    test_fn(vec!["choose", "0..2", "-c", "-x"], "abcd", "ab");
}

#[test]
fn print_0_to_2_character_wise_with_output_delimeter_rust_syntax_exclusive() {
    test_fn(vec!["choose", "0..2", "-c", "-o", ":"], "abcd", "a:b");
}

#[test]
fn print_after_end_character_wise_rust_syntax_exclusive() {
    test_fn(vec!["choose", "0..9", "-c"], "abcd", "abcd");
}

#[test]
fn print_2_to_0_character_wise_rust_syntax_exclusive() {
    test_fn(vec!["choose", "2..0", "-c"], "abcd", "ba");
}

#[test]
fn print_neg_2_to_end_character_wise_rust_syntax_exclusive() {
    test_fn(vec!["choose", "-2..", "-c"], "abcd", "cd");
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

#[test]
fn print_negative_on_empty_line() {
    test_fn(vec!["choose", "-1"], "", "");
}

#[test]
fn print_neg_2_when_field_empty() {
    test_fn(vec!["choose", "-2"], "a", "");
}

#[test]
fn print_neg_4_to_neg_2_when_fields_empty() {
    test_fn(vec!["choose", "-4:-2"], "a", "");
}

#[test]
fn print_neg_1_when_field_not_empty() {
    test_fn(vec!["choose", "-1"], "a", "a");
}

#[test]
fn print_neg_2_when_field_not_empty() {
    test_fn(vec!["choose", "-2"], "a b", "a");
}

#[test]
fn print_neg_4_to_neg_2_when_fields_not_empty() {
    test_fn(vec!["choose", "-4:-2"], "a b c d e", "b c d");
}

#[test]
fn print_before_to_before_negative() {
    test_fn(vec!["choose", "-8:-6"], "a b c d e", "");
}

#[test]
fn print_before_to_0() {
    test_fn(vec!["choose", "-8:0"], "a b c d e", "a");
}

#[test]
fn print_before_to_middle() {
    test_fn(vec!["choose", "-8:2"], "a b c d e", "a b c");
}

#[test]
fn print_before_to_after() {
    test_fn(vec!["choose", "-6:10"], "a b c d e", "a b c d e");
}

#[test]
fn print_before_to_end_negative() {
    test_fn(vec!["choose", "-6:-1"], "a b c d e", "a b c d e");
}

#[test]
fn print_middle_to_end_negative() {
    test_fn(vec!["choose", "2:-1"], "a b c d e", "c d e");
}

#[test]
fn print_middle_to_after() {
    test_fn(vec!["choose", "-3:10"], "a b c d e", "c d e");
}

#[test]
fn print_after_to_after() {
    test_fn(vec!["choose", "10:10"], "a b c d e", "");
}

#[test]
fn print_negative_end_to_negative_end() {
    test_fn(vec!["choose", "-1:-1"], "a b c d e", "e");
}
//////
#[test]
fn print_before_to_before_negative_empty() {
    test_fn(vec!["choose", "-8:-6"], "", "");
}

#[test]
fn print_before_to_0_empty() {
    test_fn(vec!["choose", "-8:0"], "", "");
}

#[test]
fn print_before_to_middle_empty() {
    test_fn(vec!["choose", "-8:2"], "", "");
}

#[test]
fn print_before_to_after_empty() {
    test_fn(vec!["choose", "-6:10"], "", "");
}

#[test]
fn print_before_to_end_negative_empty() {
    test_fn(vec!["choose", "-6:-1"], "", "");
}

#[test]
fn print_middle_to_end_negative_empty() {
    test_fn(vec!["choose", "2:-1"], "", "");
}

#[test]
fn print_middle_to_after_empty() {
    test_fn(vec!["choose", "-3:10"], "", "");
}

#[test]
fn print_after_to_after_empty() {
    test_fn(vec!["choose", "10:10"], "", "");
}

#[test]
fn print_negative_end_to_negative_end_empty() {
    test_fn(vec!["choose", "-1:-1"], "", "");
}

#[test]
fn print_positive_to_following_negative() {
    test_fn(vec!["choose", "1:-3"], "a b c d e", "b c");
}

#[test]
fn print_positive_to_same_as_negative() {
    test_fn(vec!["choose", "1:-4"], "a b c d e", "b");
}

#[test]
fn print_positive_to_preceding_negative() {
    test_fn(vec!["choose", "1:-5"], "a b c d e", "");
}

#[test]
fn print_end_to_last_negative_is_last() {
    test_fn(vec!["choose", "4:-1"], "a b c d e", "e");
}

#[test]
fn print_after_end_to_last_negative_is_empty() {
    test_fn(vec!["choose", "5:-1"], "a b c d e", "");
}

#[test]
fn print_after_end_to_second_to_last_negative_is_empty() {
    test_fn(vec!["choose", "5:-2"], "a b c d e", "");
}

#[test]
fn do_not_print_carriage_return() {
    test_fn(vec!["choose", ":"], "ABC;GHI;JKKK;KLLL\r  ", "ABC;GHI;JKKK;KLLL");
}
