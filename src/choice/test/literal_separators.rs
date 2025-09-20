use crate::{choice::test::MockStdout, config::Config, process_all_choices_for_line, writer::Writer};

use super::*;

fn test_fn(vec: Vec<&str>, input: &str, output: &str) {
    let config = Config::from_iter(vec);
    let mut handle = Writer::from(BufWriter::new(MockStdout::new()));

    process_all_choices_for_line(&mut handle, &config, input).unwrap();

    assert_eq!(String::from(output), MockStdout::str_from_writer(handle));
}

#[test]
fn test_bang() {
    test_fn(vec!["choose", "-f", "!", "1"], "rust!is!pretty!cool", "is");
}

#[test]
fn test_caret() {
    test_fn(vec!["choose", "-f", "\\^", "1"], "rust^is^pretty^cool", "is");
}

#[test]
fn test_paren() {
    test_fn(vec!["choose", "-f", "\\(", "1"], "rust(is(pretty(cool", "is");
}
