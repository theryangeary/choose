use super::*;
use crate::Error;

#[test]
fn positive_negative_1() {
    let config = Config::from_iter(vec!["choose", "2:-1"]);
    let slice = &[1, 2, 3, 4, 5];
    assert_eq!(
        Some((2, 4)),
        config.opt.choices[0].get_negative_start_end(slice).unwrap()
    );
}

#[test]
fn positive_negative_gt1() {
    let config = Config::from_iter(vec!["choose", "1:-3"]);
    let slice = &[1, 2, 3, 4, 5];
    assert_eq!(
        Some((1, 2)),
        config.opt.choices[0].get_negative_start_end(slice).unwrap()
    );
}

#[test]
fn negative_positive() {
    let config = Config::from_iter(vec!["choose", "-3:4"]);
    let slice = &[1, 2, 3, 4, 5];
    assert_eq!(
        Some((2, 4)),
        config.opt.choices[0].get_negative_start_end(slice).unwrap()
    );
}

#[test]
fn negative_negative() {
    let config = Config::from_iter(vec!["choose", "-3:-4"]);
    let slice = &[1, 2, 3, 4, 5];
    assert_eq!(
        Some((2, 1)),
        config.opt.choices[0].get_negative_start_end(slice).unwrap()
    );
}

#[test]
fn negative1_negative1() {
    let config = Config::from_iter(vec!["choose", "-1:-1"]);
    let slice = &[1, 2, 3, 4, 5];
    assert_eq!(
        Some((4, 4)),
        config.opt.choices[0].get_negative_start_end(slice).unwrap()
    );
}

#[test]
fn negative_nonexisting_positive() {
    let config = Config::from_iter(vec!["choose", "-3:9"]);
    let slice = &[1, 2, 3, 4, 5];
    assert_eq!(
        Some((2, 4)),
        config.opt.choices[0].get_negative_start_end(slice).unwrap()
    );
}

#[test]
fn negative_negative_on_empty() {
    let config = Config::from_iter(vec!["choose", "-3:-1"]);
    let slice = &[0u8; 0];
    assert_eq!(
        None,
        config.opt.choices[0].get_negative_start_end(slice).unwrap()
    )
}

#[test]
fn negative_positive_on_empty() {
    let config = Config::from_iter(vec!["choose", "-3:5"]);
    let slice = &[0u8; 0];
    assert_eq!(
        None,
        config.opt.choices[0].get_negative_start_end(slice).unwrap()
    )
}

#[test]
fn positive_negative_on_empty() {
    let config = Config::from_iter(vec!["choose", "2:-1"]);
    let slice = &[0u8; 0];
    assert_eq!(
        None,
        config.opt.choices[0].get_negative_start_end(slice).unwrap()
    )
}

#[test]
fn negative_positive_all() {
    let config = Config::from_iter(vec!["choose", "-5:9"]);
    let slice = &[0, 1, 2, 3];
    assert_eq!(
        Some((0, 3)),
        config.opt.choices[0].get_negative_start_end(slice).unwrap()
    );
}

#[test]
fn negative_positive_some() {
    let config = Config::from_iter(vec!["choose", "-5:2"]);
    let slice = &[0, 1, 2, 3];
    assert_eq!(
        Some((0, 2)),
        config.opt.choices[0].get_negative_start_end(slice).unwrap()
    );
}

#[test]
fn positive_negative_all() {
    let config = Config::from_iter(vec!["choose", "9:-5"]);
    let slice = &[0, 1, 2, 3];
    assert_eq!(
        Some((3, 0)),
        config.opt.choices[0].get_negative_start_end(slice).unwrap()
    );
}

#[test]
fn positive_negative_some() {
    let config = Config::from_iter(vec!["choose", "9:-2"]);
    let slice = &[0, 1, 2, 3];
    assert_eq!(
        Some((3, 2)),
        config.opt.choices[0].get_negative_start_end(slice).unwrap()
    );
}

#[test]
fn positive_negative_same() {
    let config = Config::from_iter(vec!["choose", "1:-3"]);
    let slice = &[0, 1, 2, 3];
    assert_eq!(
        Some((1, 1)),
        config.opt.choices[0].get_negative_start_end(slice).unwrap()
    );
}

#[test]
fn error_when_choice_is_isize_min() {
    let isize_min = format!("{}", isize::MIN);
    let config = Config::from_iter(vec!["choose", &isize_min]);
    let slice = &[0, 1, 2, 3];

    let err = config.opt.choices[0]
        .get_negative_start_end(slice)
        .unwrap_err();

    if let Error::Config(s) = err {
        assert!(s.contains("Minimum index value supported is isize::MIN"));
    } else {
        panic!("Expected Error::Config, found {}", err)
    }
}

#[test]
fn error_when_choice_start_is_isize_min() {
    let choice = format!("{}:4", isize::MIN);
    let config = Config::from_iter(vec!["choose", &choice]);
    let slice = &[0, 1, 2, 3];

    let err = config.opt.choices[0]
        .get_negative_start_end(slice)
        .unwrap_err();

    if let Error::Config(s) = err {
        assert!(s.contains("Minimum index value supported is isize::MIN"));
    } else {
        panic!("Expected Error::Config, found {}", err)
    }
}

#[test]
fn error_when_choice_end_is_isize_min() {
    let choice = format!("4:{}", isize::MIN);
    let config = Config::from_iter(vec!["choose", &choice]);
    let slice = &[0, 1, 2, 3];

    let err = config.opt.choices[0]
        .get_negative_start_end(slice)
        .unwrap_err();

    if let Error::Config(s) = err {
        assert!(s.contains("Minimum index value supported is isize::MIN"));
    } else {
        panic!("Expected Error::Config, found {}", err)
    }
}
