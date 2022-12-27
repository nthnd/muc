use crate::Args;
use aecir::style::{Color, ColorName, Format};
use std::collections::HashMap;
use std::io::{BufRead, BufReader};

pub fn get_contents(args: &Args) -> String {
    let histfile_buffer = std::fs::File::open(&args.file).unwrap();
    let reader = BufReader::new(histfile_buffer);
    let mut contents = String::new();

    for (index, line) in reader.lines().enumerate() {
        if let Ok(line) = line {
            contents.push_str(&line);
            contents.push_str("\n");
        } else {
            if args.debug {
                println!(
                    "{yellow}{bold}[Error]{reset}Could not read line : {index} = {line:#?}",
                    yellow = Color::Fg(ColorName::Yellow),
                    bold = Format::Bold, 
                    reset = aecir::style::reset_all()
                );
            }
        }
    }

    contents
}

pub fn parse_contents(contents: String, prefix: &Option<String>) -> HashMap<String, usize> {
    let commands: Vec<&str> = contents
        .split(&['\n', '&', '|', ';'])
        .filter(|x| !x.is_empty())
        .into_iter()
        .map(|command| command.split_whitespace().next().unwrap())
        .map(|command| {
            if let Some(pfx) = &prefix {
                if command.starts_with(pfx) {
                    &command[0..(pfx.len())]
                } else {
                    command
                }
            } else {
                command
            }
        })
        .collect();

    let mut with_frequencies = HashMap::new();

    for command in commands.into_iter() {
        *with_frequencies.entry(command.into()).or_insert(0) += 1;
    }

    with_frequencies
}
