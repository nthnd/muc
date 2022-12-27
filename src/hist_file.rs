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

/// Find the index of the first occurrence of `target` but takes into account
/// escaping made with back slashes.
fn find_unescaped(contents: &[char], target: char) -> Option<usize> {
    let mut index = 0;
    let mut escaped = false;
    for &c in contents {
        index += 1;
        if escaped {
            escaped = false;
        } else if c == '\\' {
            escaped = true;
        } else if c == target {
            return Some(index-1);
        }
    }
    None
}

/// Removes all quotes strings put in between the given delimiters.
/// For example:
/// ```
/// remove_quoted_strings(Hi "Mike \" Ventury"!) => Hi!
/// ```
fn remove_quoted_strings(contents: String, delimiter: char) -> String {

    fn _remove_quoted_strings(contents: Vec<char>, delimiter: char) -> Vec<char> {
        match find_unescaped(&contents, delimiter) {
            Some(fist_match) => {
                let ret = &contents[0..fist_match];
                match find_unescaped(&contents[fist_match+1..], delimiter) {
                    Some(second_match) => {
                        let rest = &contents[fist_match+second_match+2..];
                        let mut concat: Vec<char> = vec![];
                        concat.extend_from_slice(ret);
                        concat.extend_from_slice(rest);
                        _remove_quoted_strings(concat, delimiter)
                    },
                    None => contents
                }
            },
            None => contents,
        }
    }

    let contents_ca: Vec<char> = contents.chars().collect();
    _remove_quoted_strings(contents_ca, delimiter).iter().collect()
}

/// Removes all quotes strings from the input
fn remove_all_quotes(contents: String) -> String {
    remove_quoted_strings(
        remove_quoted_strings(
            remove_quoted_strings(contents, '`'),
            '"'),
            '\'')
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
    let unquoted = remove_all_quotes(only_prefix);
    let commands: Vec<&str> = unquoted
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
