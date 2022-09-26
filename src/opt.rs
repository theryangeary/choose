use std::path::PathBuf;

use crate::choice::Choice;
use crate::parse;

#[derive(Debug, clap::Parser)]
#[clap(
    name = "choose",
    version,
    about = "`choose` sections from each line of files"
)]
#[clap(setting = clap::AppSettings::AllowLeadingHyphen)]
pub struct Opt {
    /// Choose fields by character number
    #[clap(short, long)]
    pub character_wise: bool,

    /// Activate debug mode
    #[clap(short, long)]
    pub debug: bool,

    /// Use exclusive ranges, similar to array indexing in many programming languages
    #[clap(short = 'x', long)]
    pub exclusive: bool,

    /// Specify field separator other than whitespace, using Rust `regex` syntax
    #[clap(short, long)]
    pub field_separator: Option<String>,

    /// Input file
    #[clap(short, long, parse(from_os_str))]
    pub input: Option<PathBuf>,

    /// Use non-greedy field separators
    #[clap(short, long)]
    pub non_greedy: bool,

    /// Index from 1 instead of 0
    #[clap(long)]
    pub one_indexed: bool,

    /// Specify output field separator
    #[clap(short, long, parse(from_str = parse::output_field_separator))]
    pub output_field_separator: Option<String>,

    /// Fields to print. Either a, a:b, a..b, or a..=b, where a and b are integers. The beginning
    /// or end of a range can be omitted, resulting in including the beginning or end of the line,
    /// respectively. a:b is inclusive of b (unless overridden by -x). a..b is
    /// exclusive of b and a..=b is inclusive of b.
    #[clap(required = true, min_values = 1, parse(try_from_str = parse::choice))]
    pub choices: Vec<Choice>,
}
