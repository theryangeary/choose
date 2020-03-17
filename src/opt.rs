use std::path::PathBuf;
use structopt::StructOpt;

use crate::choice::Choice;
use crate::config::Config;

#[derive(Debug, StructOpt)]
#[structopt(name = "choose", about = "`choose` sections from each line of files")]
#[structopt(setting = structopt::clap::AppSettings::AllowLeadingHyphen)]
pub struct Opt {
    /// Specify field separator other than whitespace
    #[structopt(short, long)]
    pub field_separator: Option<String>,

    /// Use exclusive ranges, similar to array slicing in many programming languages
    #[structopt(short = "x", long)]
    pub exclusive: bool,

    /// Activate debug mode
    #[structopt(short, long)]
    pub debug: bool,

    /// Input file
    #[structopt(short, long, parse(from_os_str))]
    pub input: Option<PathBuf>,

    /// Fields to print. Either x, x:, :y, or x:y, where x and y are integers, colons indicate a
    /// range, and an empty field on either side of the colon continues to the beginning or end of
    /// the line.
    #[structopt(required = true, min_values = 1, parse(try_from_str = Config::parse_choice))]
    pub choice: Vec<Choice>,
}
