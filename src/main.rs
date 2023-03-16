mod args;
mod display;
mod hist_file;
use std::{env, path::PathBuf};

use args::Args;

use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let file_path = match args.file.as_ref() {
        Some(p) => p.to_owned(),
        None => match env::var("HISTFILE") {
            Ok(p) => PathBuf::from(p),
            Err(_e) => {
                return Err("Could not find HISTFILE environment variable. Supply the file path with the --file option".into());
            }
        },
    };

    let file = std::fs::File::open(file_path).expect("Failed to get histfile");

    let contents = hist_file::get_contents(file, &args);
    let command_lines = hist_file::parse_contents(contents, &args);
    let commands = hist_file::process_lines(command_lines, &args);

    display::print(commands, args);

    Ok(())
}
