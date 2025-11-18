use super::*;

#[test]
fn is_field_reversed() {
    let config = Config::from_iter(vec!["choose", "0"]);
    assert!(!config.opt.choices[0].is_reverse_range());
}

#[test]
fn is_field_range_no_start_reversed() {
    let config = Config::from_iter(vec!["choose", ":2"]);
    assert!(!config.opt.choices[0].is_reverse_range());
}

#[test]
fn is_field_range_no_end_reversed() {
    let config = Config::from_iter(vec!["choose", "2:"]);
    assert!(!config.opt.choices[0].is_reverse_range());
}

#[test]
fn is_field_range_no_start_or_end_reversed() {
    let config = Config::from_iter(vec!["choose", ":"]);
    assert!(!config.opt.choices[0].is_reverse_range());
}

#[test]
fn is_reversed_field_range_reversed() {
    let config = Config::from_iter(vec!["choose", "4:2"]);
    assert!(config.opt.choices[0].is_reverse_range());
}
