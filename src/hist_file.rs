use crate::Args;
use aecir::style::{Color, ColorName, Format};
use std::collections::HashMap;
use std::io::{BufRead, BufReader};

fn print_warning(warning: &str) {
    println!(
        "{yellow}{bold}[Error]{reset} {warning}",
        yellow = Color::Fg(ColorName::Yellow),
        bold = Format::Bold,
        reset = aecir::style::reset_all()
    );
}

pub fn get_contents(args: &Args) -> String {
    let Ok(histfile_buffer) = std::fs::File::open(&args.file) else { panic!("Please specify a valid histfile")};

    let reader = BufReader::new(histfile_buffer);
    let mut contents = String::new();

    for (index, line) in reader.lines().enumerate() {
        if let Ok(line) = line {
            contents.push_str(&line);
            contents.push('\n');
        } else if args.debug {
            print_warning(&format!("Could not read line : {index} = {line:#?}"));
        }
    }

    contents
}

pub fn parse_contents(contents: String, args: &Args) -> HashMap<String, usize> {
    let commands: Vec<&str> = contents
        .split(&['\n', '&', '|', ';'])
        .filter(|x| !x.is_empty())
        .into_iter()
        .map(|command| {
            command.split_whitespace().next().unwrap_or_else(|| {
                if args.debug {
                    print_warning("Error while parsing command");
                }
                ""
            })
        })
        .map(|command| match &args.prefix {
            Some(pfx) if command.starts_with(pfx) => &command[0..pfx.len()],
            _ => command,
        })
        .collect();

    let mut with_frequencies = HashMap::new();

    for command in commands.into_iter() {
        *with_frequencies.entry(command.into()).or_insert(0) += 1;
    }

    with_frequencies
}
