use std::path::PathBuf;
use structopt::StructOpt;

use crate::choice::Choice;
use crate::parse;

#[derive(Debug, StructOpt)]
#[structopt(name = "choose", about = "`choose` sections from each line of files")]
#[structopt(setting = structopt::clap::AppSettings::AllowLeadingHyphen)]
pub struct Opt {
    /// Choose fields by character number
    #[structopt(short, long)]
    pub character_wise: bool,

    /// Activate debug mode
    #[structopt(short, long)]
    #[allow(unused)]
    pub debug: bool,

    /// Use exclusive ranges, similar to array indexing in many programming languages
    #[structopt(short = "x", long)]
    pub exclusive: bool,

    /// Specify field separator other than whitespace, using Rust `regex` syntax
    #[structopt(short, long)]
    pub field_separator: Option<String>,

    /// Input file
    #[structopt(short, long, parse(from_os_str))]
    pub input: Option<PathBuf>,

    /// Use non-greedy field separators
    #[structopt(short, long)]
    pub non_greedy: bool,

    /// Index from 1 instead of 0
    #[structopt(long)]
    pub one_indexed: bool,

    /// Specify output field separator
    #[structopt(short, long, parse(from_str = parse::output_field_separator))]
    pub output_field_separator: Option<String>,

    /// Fields to print. Either a, a:b, a..b, or a..=b, where a and b are integers. The beginning
    /// or end of a range can be omitted, resulting in including the beginning or end of the line,
    /// respectively. a:b is inclusive of b (unless overridden by -x). a..b is
    /// exclusive of b and a..=b is inclusive of b.
    #[structopt(required = true, min_values = 1, parse(try_from_str = parse::choice))]
    pub choices: Vec<Choice>,
}
