// use std::env;
#![allow(unused)]

mod hist_file;
mod utils;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the file where history is stored
    #[arg(short, long)]
    file: String,

    /// Specify if the file contains any prefixes
    #[arg(short, long)]
    prefix: Option<String>,

    /// How many of the top commands to show
    #[arg(short, long)]
    count: Option<usize>,
}

fn main() {
    let args = Args::parse();
    let commands = hist_file::parse_contents(hist_file::get_contents(args.file), args.prefix);
    utils::display_sorted(commands, args.count);
}
