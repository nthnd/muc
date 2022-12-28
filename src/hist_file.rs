use crate::Args;
use aecir::style::{Color, ColorName, Format};
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use regex::Regex;

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
    let separators: Vec<char> = args.separators.chars().collect();
    let mut only_prefix = "".to_string();
    for line in contents.split('\n') {
        only_prefix.push_str(match &args.prefix {
            Some(pfx) => {
                if line.starts_with(pfx) {
                    &line[pfx.len()..]
                } else {
                    ""
                }
            },
            _ => line,
        });
        only_prefix.push_str("\n");
    }

    let regexp = match args.shell.to_lowercase().as_str() {
        "bash" => &"",
        "zsh" => &r": \d\d\d\d\d\d\d\d\d\d:\d;",
        "fish" => &"- cmd: ",
        _ => args.regexp.as_str(),
    };

    let reg = Regex::new(&regexp).unwrap();
    only_prefix = reg.replace_all(&only_prefix, "").to_string();


    let commands: Vec<&str> = only_prefix
        .split(&*separators)
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
        .collect();

    let mut with_frequencies = HashMap::new();

    for command in commands.into_iter() {
        *with_frequencies.entry(command.into()).or_insert(0) += 1;
    }

    with_frequencies
}
