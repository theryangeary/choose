use bpaf::Bpaf;
use std::path::PathBuf;

use crate::choice::Choice;
use crate::parse::output_field_separator;

#[derive(Debug, Clone, Bpaf)]
/// `choose` sections from each line of files
#[bpaf(options, generate(options))]
pub struct Opt {
    /// Choose fields by character number
    #[bpaf(short, long)]
    pub character_wise: bool,

    /// Activate debug mode
    #[bpaf(short, long)]
    pub debug: bool,

    /// Use exclusive ranges, similar to array indexing in many programming languages
    #[bpaf(short('x'), long)]
    pub exclusive: bool,

    /// Specify field separator other than whitespace, using Rust `regex` syntax
    #[bpaf(short, long)]
    pub field_separator: Option<String>,

    /// Input file
    #[bpaf(short, long, argument("INPUT"), optional)]
    pub input: Option<PathBuf>,

    /// Use non-greedy field separators
    #[bpaf(short, long)]
    pub non_greedy: bool,

    /// Index from 1 instead of 0
    #[bpaf(long)]
    pub one_indexed: bool,

    /// Specify output field separator
    #[bpaf(short, long, argument("SEP"), map(output_field_separator), optional)]
    pub output_field_separator: Option<String>,

    /// Fields to print. Either a, a:b, a..b, or a..=b, where a and b are integers. The beginning
    /// or end of a range can be omitted, resulting in including the beginning or end of the line,
    /// respectively. a:b is inclusive of b (unless overridden by -x). a..b is
    /// exclusive of b and a..=b is inclusive of b.
    #[bpaf(any::<Choice>("CHOICE"), some("At least one is required"))]
    pub choices: Vec<Choice>,
}
