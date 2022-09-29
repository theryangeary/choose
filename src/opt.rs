use std::path::PathBuf;

use crate::choice::Choice;
use crate::parse;

#[derive(clap::Parser)]
#[command(version, about = "`choose` sections from each line of files")]
pub struct Opt {
    /// Choose fields by character number
    #[arg(short, long)]
    pub character_wise: bool,

    /// Activate debug mode
    #[arg(short, long)]
    pub debug: bool,

    /// Use exclusive ranges, similar to array indexing in many programming languages
    #[arg(short = 'x', long)]
    pub exclusive: bool,

    /// Specify field separator other than whitespace, using Rust `regex` syntax
    #[arg(short, long)]
    pub field_separator: Option<String>,

    /// Input file
    #[arg(short, long)]
    pub input: Option<PathBuf>,

    /// Use non-greedy field separators
    #[arg(short, long)]
    pub non_greedy: bool,

    /// Index from 1 instead of 0
    #[arg(long)]
    pub one_indexed: bool,

    /// Specify output field separator
    #[arg(short, long, value_parser = parse::output_field_separator)]
    pub output_field_separator: Option<String>,

    /// Fields to print. Either a, a:b, a..b, or a..=b, where a and b are integers. The beginning
    /// or end of a range can be omitted, resulting in including the beginning or end of the line,
    /// respectively. a:b is inclusive of b (unless overridden by -x). a..b is
    /// exclusive of b and a..=b is inclusive of b.
    #[arg(allow_hyphen_values = true, value_parser = parse::choice)]
    pub choices: Vec<Choice>,
}
