mod hist_file;
mod utils;
mod args;
use std::env;

use args::Args;

use clap::Parser;

fn main() {
    let args = Args::parse();

    let file_path = env::var("HISTFILE").expect("HISTFILE not set");
    let file = std::fs::File::open(file_path).expect("Failed to get histfile");

    let contents = hist_file::get_contents(file, &args);
    let command_lines = hist_file::parse_contents(contents, &args);
    let commands = hist_file::process_lines(command_lines, &args);


    utils::display_sorted(commands, args);
}
