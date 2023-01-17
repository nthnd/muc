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
    let histfile_buffer = std::fs::File::open(&args.file).expect("Couldn't find histfile");

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

    fn _remove_quoted_strings<'a>(contents: &'a[char], delimiter: char, slices: &mut Vec<&'a [char]>) {
        if let Some(first_match) = find_unescaped(&contents, delimiter) {
            let ret = &contents[0..first_match];
            slices.push(ret);
            if let Some(second_match) = find_unescaped(&contents[first_match+1..], delimiter) {
                let rest = &contents[first_match+second_match+2..];
                _remove_quoted_strings(rest, delimiter, slices);
            } else {
                slices.push(&contents[first_match+1..]);
            }
        } else {
            slices.push(&contents);
        }
    }

    let mut all_matches: Vec<&[char]> = vec![];
    let contents_ca: Vec<char> = contents.chars().collect();
    _remove_quoted_strings(&contents_ca, delimiter, &mut all_matches);

    let mut concat: Vec<char> = vec![];
    for slice in all_matches {
        concat.extend_from_slice(slice);
    }
    concat.iter().collect()
}

/// Removes all quotes strings from the input
fn remove_all_quotes(contents: &str) -> String {
    remove_quoted_strings(
        remove_quoted_strings(
            remove_quoted_strings(contents.to_string(), '`'),
            '"'),
            '\'')
}

pub fn parse_contents(contents: String, args: &Args) 
    -> ( HashMap<String, usize>, HashMap<String, Vec<String>> ) {
    let separators: Vec<char> = args.separators.chars().collect();
    let mut only_prefix = "".to_string();

    let prefix = match &args.prefix {
        Some(pfx) => pfx,
        None => match args.shell.as_str() {
            "fish" => "- cmd: ",
            _ => "",
        },
    };

    for line in contents.split('\n') {
        only_prefix.push_str(match prefix {
            "" => line,
            pfx => {
                if line.starts_with(pfx) {
                    &line[pfx.len()..]
                } else {
                    ""
                }
            },
        });
        only_prefix.push('\n');
    }

    let mut unquoted = remove_all_quotes(&only_prefix);


    let regexp = match args.shell.to_lowercase().as_str() {
        "bash" => "",
        "zsh" => r": \d\d\d\d\d\d\d\d\d\d:\d;",
        "fish" => match &args.prefix {
            Some(_pfx) =>"- cmd: ",
            None => "", // If no prefix had been given, the default prefix already deleted the "- cmd: "s
        }
        _ => args.regexp.as_str(),
    };

    let reg = Regex::new(regexp).unwrap();
    unquoted = reg.replace_all(&unquoted, "").to_string();


    let command_lines: Vec<&str> = unquoted
        .split(&*separators)
        .filter(|x| !x.is_empty()).collect();

    let mut commands: Vec<&str> = Vec::new();
    let mut sub_commands: HashMap<String, Vec<String>> = HashMap::new();

    for command in command_lines.iter() {
        let mut words = command.split_whitespace();
        if let Some(first_word) = words.next() {
            // true means ignore the sub command
            let leaders = vec![
                ( "sudo", false ),
                ( "watch", false ),
                ( "git", true),
                ( "cargo", true ),
            ];
            commands.push(first_word);
            leaders.iter().for_each(|(leader, should_ignore)| {
                if leader == &first_word {
                    if let Some(second_word) = words.next() {
                        if !should_ignore { commands.push(second_word) }
                        sub_commands.entry(first_word.into()).and_modify(|arr| {
                            if !arr.contains(&second_word.to_owned()){
                                arr.push(second_word.into());
                            }
                        }).or_insert_with(|| vec![second_word.into()]);
                    }
                }

            });
        }
        else if args.debug {
                print_warning("Error while parsing command");
        }
    }

    let mut with_frequencies = HashMap::new();

    for command in commands.into_iter() {
        *with_frequencies.entry(command.into()).or_insert(0) += 1;
    }

    (with_frequencies, sub_commands)
}

