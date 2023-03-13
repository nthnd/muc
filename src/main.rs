mod hist_file;
mod utils;
mod args;
use args::Args;

use clap::Parser;

fn main() {
    let args = Args::parse();
    let contents = hist_file::get_contents(&args);
    let command_lines = hist_file::parse_contents(contents, &args);
    let commands = hist_file::process_lines(command_lines, &args);


    utils::display_sorted(commands, args);
}
