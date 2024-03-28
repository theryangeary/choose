use std::path::PathBuf;
use std::process;

use crate::choice::Choice;
use crate::parse::{self, choice, PARSE_CHOICE_RE};

pub struct Opt {
    /// Choose fields by character number
    pub character_wise: bool,

    /// Activate debug mode
    pub debug: bool,

    /// Use exclusive ranges, similar to array indexing in many programming languages
    pub exclusive: bool,

    /// Specify field separator other than whitespace, using Rust `regex` syntax
    pub field_separator: Option<String>,

    /// Input file
    pub input: Option<PathBuf>,

    /// Use non-greedy field separators
    pub non_greedy: bool,

    /// Index from 1 instead of 0
    pub one_indexed: bool,

    /// Specify output field separator
    pub output_field_separator: Option<String>,

    /// Fields to print. Either a, a:b, a..b, or a..=b, where a and b are integers. The beginning
    /// or end of a range can be omitted, resulting in including the beginning or end of the line,
    /// respectively. a:b is inclusive of b (unless overridden by -x). a..b is
    /// exclusive of b and a..=b is inclusive of b.
    pub choices: Vec<Choice>,
}

const LONG_ARGS: &[&str] = &[
    "character-wise",
    "debug",
    "exclusive",
    "field-separator",
    "input",
    "non-greedy",
    "one-indexed",
    "output-field-separator",
];

const SHORT_ARGS: &[&str] = &["c", "d", "x", "f", "i", "n", "", "o"];

macro_rules! HELP {
    () => {
r"{} {}
`choose` sections from each line of files

USAGE:
    choose [FLAGS] [OPTIONS] <choices>...

FLAGS:
    -c, --character-wise    Choose fields by character number
    -d, --debug             Activate debug mode
    -x, --exclusive         Use exclusive ranges, similar to array indexing in many programming languages
    -h, --help              Prints help information
    -n, --non-greedy        Use non-greedy field separators
        --one-indexed       Index from 1 instead of 0
    -V, --version           Prints version information

OPTIONS:
    -f, --field-separator <field-separator>
            Specify field separator other than whitespace, using Rust `regex` syntax

    -i, --input <input>                                      Input file
    -o, --output-field-separator <output-field-separator>    Specify output field separator

ARGS:
    <choices>...    Fields to print. Either a, a:b, a..b, or a..=b, where a and b are integers. The beginning or end
                    of a range can be omitted, resulting in including the beginning or end of the line,
                    respectively. a:b is inclusive of b (unless overridden by -x). a..b is exclusive of b and a..=b
                    is inclusive of b
        "
    };
}

impl Opt {
    pub fn new(args: Vec<&str>) -> Self {
        let mut choices = vec![];
        let mut character_wise = false;
        let mut debug = false;
        let mut exclusive = false;
        let mut field_separator = None;
        let mut input = None;
        let mut non_greedy = false;
        let mut one_indexed = false;
        let mut output_field_separator = None;

        let mut argument_value = false;

        for (i, c) in args.iter().enumerate() {
            if argument_value {
                if c.starts_with("-") {
                    panic!("Argument {} has no value", args[i - 1]);
                }

                argument_value = false;
                continue;
            }

            if is_choice(c) {
                let choice = choice(c).expect(&format!("Failed to parse choice {}", c));
                choices.push(choice);
                continue;
            }

            match c.strip_prefix("--") {
                Some(c) if !LONG_ARGS.contains(&c) => panic!("Arg {} does not support.", c),
                Some(c) => {
                    match c {
                        "character-wise" => character_wise = true,
                        "debug" => debug = true,
                        "exclusive" => exclusive = true,
                        "field-separator" => {
                            argument_value = true;
                            let value = args.get(i + 1).expect(&format!("Arg {} has no value", c));
                            field_separator = Some(value.to_string());
                        }
                        "input" => {
                            argument_value = true;
                            let value = args.get(i + 1).expect(&format!("Arg {} has no value", c));
                            input = Some(PathBuf::from(value));
                        }
                        "non-greedy" => non_greedy = true,
                        "one-indexed" => one_indexed = true,
                        "output-field-separator" => {
                            argument_value = true;
                            let value = args.get(i + 1).expect(&format!("Arg {} has no value", c));
                            output_field_separator = Some(parse::output_field_separator(value).unwrap());
                        }
                        _ => unreachable!(),
                    }
                    continue;
                }
                None => {}
            }

            match c.strip_prefix("-") {
                Some(c) if !SHORT_ARGS.contains(&c) => panic!("Arg {} does not support.", c),
                Some(c) => {
                    match c {
                        "c" => character_wise = true,
                        "d" => debug = true,
                        "x" => exclusive = true,
                        "f" => {
                            argument_value = true;
                            let value = args.get(i + 1).expect(&format!("Arg {} has no value", c));
                            field_separator = Some(value.to_string());
                        }
                        "i" => {
                            argument_value = true;
                            let value = args.get(i + 1).expect(&format!("Arg {} has no value", c));
                            input = Some(PathBuf::from(value));
                        }
                        "n" => non_greedy = true,
                        "o" => {
                            argument_value = true;
                            let value = args.get(i + 1).expect(&format!("Arg {} has no value", c));
                            output_field_separator = Some(parse::output_field_separator(value).unwrap());
                        }
                        _ => unreachable!(),
                    }
                    continue;
                }
                None => {}
            }
        }

        if choices.is_empty() {
            println!(HELP!(), env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
            process::exit(1);
        }

        Self {
            character_wise,
            debug,
            exclusive,
            field_separator,
            input,
            non_greedy,
            one_indexed,
            output_field_separator,
            choices,
        }
    }

    pub fn parse() -> Self {
        let args = std::env::args().collect::<Vec<_>>();

        if args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) {
            println!(HELP!(),  env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
            process::exit(0);
        }

        if args.contains(&"--version".to_string()) || args.contains(&"-V".to_string()) {
            println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
            process::exit(0);
        }

        Self::new(args.iter().map(|x| x.as_str()).collect::<Vec<_>>())
    }
}

fn is_choice(arg: &str) -> bool {
    PARSE_CHOICE_RE.captures_iter(arg).next().is_some() || arg.parse::<isize>().is_ok()
}
