pub fn process_escapes(input: &str) -> String {
    if input.len() < 1 {
        return String::from(input);
    }

    let mut v = Vec::from(input);
    for i in 0..(v.len() - 1) {
        if v[i] == '\\' as u8 && is_escapable(v[i + 1] as char) {
            v.remove(i);
            v[i] = char_to_escape_sequence(v[i] as char) as u8;
        }
    }
    String::from_utf8(v).unwrap()
}

fn char_to_escape_sequence(chr: char) -> char {
    match chr {
        'n' => '\n',
        't' => '\t',
        'r' => '\r',
        '\\' => '\\',
        '0' => '\0',
        _ => chr,
    }
}

fn is_escapable(chr: char) -> bool {
    match chr {
        'n' | 't' | 'r' | '\\' | '0' => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod test_process_escapes {
        use super::*;

        #[test]
        fn test_newline() {
            assert_eq!(
                String::from("hello\nworld"),
                process_escapes(r#"hello\nworld"#)
            );
        }

        #[test]
        fn test_carriage_return() {
            assert_eq!(
                String::from("hello\rworld"),
                process_escapes(r#"hello\rworld"#)
            );
        }

        #[test]
        fn test_tab() {
            assert_eq!(
                String::from("hello\tworld"),
                process_escapes(r#"hello\tworld"#)
            );
        }

        #[test]
        fn test_backslash() {
            assert_eq!(
                String::from("hello\\world"),
                process_escapes(r#"hello\\world"#)
            );
        }

        #[test]
        fn test_null() {
            assert_eq!(
                String::from("hello\0world"),
                process_escapes(r#"hello\0world"#)
            );
        }
    }

    mod test_char_to_escape_sequence {
        use super::*;
        #[test]
        fn test_escape_n() {
            assert_eq!('\n', char_to_escape_sequence('n'));
        }

        #[test]
        fn test_escape_t() {
            assert_eq!('\t', char_to_escape_sequence('t'));
        }

        #[test]
        fn test_escape_r() {
            assert_eq!('\r', char_to_escape_sequence('r'));
        }

        #[test]
        fn test_escape_backslash() {
            assert_eq!('\\', char_to_escape_sequence('\\'));
        }

        #[test]
        fn test_escape_0() {
            assert_eq!('\0', char_to_escape_sequence('0'));
        }
    }

    mod is_escapable_tests {
        use super::*;

        #[test]
        fn test_escape_n() {
            assert!(is_escapable('n'));
        }

        #[test]
        fn test_escape_t() {
            assert!(is_escapable('t'));
        }

        #[test]
        fn test_escape_r() {
            assert!(is_escapable('r'));
        }

        #[test]
        fn test_escape_backslash() {
            assert!(is_escapable('\\'));
        }

        #[test]
        fn test_escape_0() {
            assert!(is_escapable('0'));
        }
    }
}
