use crate::Args;
use aecir::style::{Color, ColorName, Format};
use regex::Regex;
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

fn remove_quoted_strings(content: &str, quot_char: char) -> String {
    let quoted = format!(
        "{quot_char}(\\.|[^{quot_char}])*{quot_char}",
        quot_char = quot_char
    );
    let reg = Regex::new(&quoted).unwrap();
    let cap = reg.replace_all(content, "");
    cap.to_string()
}

fn remove_all_quotes(contents: &str) -> String {
    let mut unquoted = remove_quoted_strings(contents, '`');
    unquoted = remove_quoted_strings(unquoted.as_str(), '"');
    unquoted = remove_quoted_strings(unquoted.as_str(), '\'');
    unquoted
}

pub fn parse_contents(
    contents: String,
    args: &Args,
) -> (HashMap<String, usize>, HashMap<String, Vec<String>>) {
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
            }
        });
        only_prefix.push('\n');
    }

    let mut unquoted = remove_all_quotes(&only_prefix);

    let regexp = match args.shell.to_lowercase().as_str() {
        "bash" => "",
        "zsh" => r": \d\d\d\d\d\d\d\d\d\d:\d;",
        "fish" => match &args.prefix {
            Some(_pfx) => "-cmd: ",
            None => "",
        }
        _ => args.regexp.as_str(),
    };

    let reg = Regex::new(regexp).unwrap();
    unquoted = reg.replace_all(&unquoted, "").to_string();

    let command_lines: Vec<&str> = unquoted
        .split(&*separators)
        .filter(|x| !x.is_empty())
        .collect();

    let mut commands: Vec<&str> = Vec::new();
    let mut sub_commands: HashMap<String, Vec<String>> = HashMap::new();

    for command in command_lines.iter() {
        let mut words = command.split_whitespace();
        if let Some(first_word) = words.next() {
            let leaders = vec![
                ("sudo", false),
                ("watch", false),
                ("git", true),
                ("cargo", true),
            ];
            commands.push(first_word);
            leaders.iter().for_each(|(leader, should_ignore)| {
                if leader == &first_word {
                    if let Some(second_word) = words.next() {
                        if !should_ignore {
                            commands.push(second_word)
                        }
                        sub_commands
                            .entry(first_word.into())
                            .and_modify(|arr| {
                                if !arr.contains(&second_word.to_owned()) {
                                    arr.push(second_word.into());
                                }
                            })
                            .or_insert_with(|| vec![second_word.into()]);
                    }
                }
            });
        } else if args.debug {
            print_warning("Error while parsing command");
        }
    }

    let mut with_frequencies = HashMap::new();

    for command in commands.into_iter() {
        *with_frequencies.entry(command.into()).or_insert(0) += 1;
    }

    (with_frequencies, sub_commands)
}

#[cfg(test)]
mod quotes {
    use super::*;

    #[test]
    fn basic_quotes() {
        assert_eq!(remove_quoted_strings("echo `hi`", '`'), "echo ".to_string());
        assert_eq!(
            remove_quoted_strings("echo \"hi\"", '"'),
            "echo ".to_string()
        );
        assert_eq!(
            remove_quoted_strings("echo 'hi'", '\''),
            "echo ".to_string()
        );
    }

    #[test]
    fn sequenced_quotes() {
        assert_eq!(
            remove_quoted_strings("echo 'hi' and another 'hi'", '\''),
            "echo  and another ".to_string()
        );
        assert_eq!(
            remove_quoted_strings("echo \"hi\" and another \"hi\"", '"'),
            "echo  and another ".to_string()
        );
        assert_eq!(
            remove_quoted_strings("echo `hi` and another `hi`", '`'),
            "echo  and another ".to_string()
        );
    }

    #[test]
    fn imbalanced() {
        assert_eq!(
            remove_quoted_strings("echo \"hi\" and another \"hi", '"'),
            "echo  and another \"hi".to_string()
        )
    }


    #[test]
    fn polyglots() {
        assert_eq!(
           remove_all_quotes("echo 'hi' | something \"hello\" && blah `hi`"),
           "echo  | something  && blah ".to_string()
        )
    }
}

#[cfg(test)]
mod parsing { 
    #[test]
    #[ignore = "reformat pending ..."]
    fn tood() {
        todo!()
    }
}
