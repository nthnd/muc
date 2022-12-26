// use std::env;
#![allow(unused)]

mod hist_file;
mod utils;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to the file where history is stored
    #[arg(short, long)]
    file: String,

    /// Specify a prefix for formatting lines
    #[arg(long)]
    prefix: Option<String>,

    /// Display top n commands
    #[arg(short, long)]
    count: Option<usize>,

    /// Make output pretty
    #[arg(short, long, default_value_t = false)]
    pretty: bool,

    /// Bar opening character
    #[arg(long, default_value_t = '[')]
    bar_open: char,

    /// Bar closing character
    #[arg(long, default_value_t = ']')]
    bar_close: char,

    /// Bar character
    #[arg(long, default_value_t = '▮')]
    bar: char,
}

fn main() {
    let args = Args::parse();
    let commands = hist_file::parse_contents(hist_file::get_contents(&args.file), &args.prefix);
    utils::display_sorted(commands, args);
}
