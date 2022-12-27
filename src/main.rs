mod hist_file;
mod utils;
mod args;
use args::Args;

use clap::Parser;

fn main() {
    let args = Args::parse();
    let commands = hist_file::parse_contents(hist_file::get_contents(&args), &args);
    utils::display_sorted(commands, args);
}
