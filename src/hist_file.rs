use crate::Args;
use crossterm::execute;
use crossterm::style::{SetForegroundColor, Color, SetAttribute, Attribute, Print};
use regex::Regex;
use std::io::stdout;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};

pub fn get_contents(hist_file: std::fs::File, args: &Args) -> String {
    let reader = BufReader::new(hist_file);
    let mut contents = String::new();

    for (index, line) in reader.lines().enumerate() {
        if let Ok(line) = line {
            contents.push_str(&line);
            contents.push('\n');
        } else if args.debug {
            execute!{
                stdout(),

                SetForegroundColor(Color::Yellow),
                SetAttribute(Attribute::Bold),
                Print(format!("[Error] Could not read line : {index} = {line:#?}\n")),
                SetAttribute(Attribute::Reset),
            }.unwrap();

        }
    }

    contents
}

fn get_commands(line: String) -> Vec<String> {
    line.split(&['&', '|', ';'])
        .filter(|x| !x.is_empty())
        .map(str::to_string)
        .collect()
}

/// Takes contents of a file and returns a vector of valid commands
pub fn parse_contents(contents: String, args: &Args) -> Vec<String> {
    let mut lines: Vec<String> = contents
        .lines()
        .filter(|line| !line.is_empty())
        .map(str::trim)
        .map(str::to_string)
        .collect();

    let shell_strat = match args.shell.as_str() {
        "fish" => |line: String| -> String {
            if line.starts_with("when: ") {
                "".to_owned()
            } else {
                line[7..].to_owned()
            }
        },
        "ohmyzsh" => |line: String| -> String { line[7..].to_owned() },
        _ => |line: String| -> String { line },
    };

    let regex_strat = |line: String, re: Regex| -> String {
        if let Some(cap) = re.captures(&line) {
            cap[0].to_owned()
        } else {
            String::new()
        }
    };

    if args.regexp.is_empty() {
        lines = lines.into_iter().map(shell_strat).collect();
    } else {
        let re = Regex::new(&args.regexp).unwrap();
        lines = lines
            .into_iter()
            .map(move |line| regex_strat(line, re.clone()))
            .collect();
    };

    let reg = Regex::new("('(?:.|[^'\n])*'|\"(?:.|[^\"\n])*\")").unwrap();
    let unquoted_lines = lines
        .into_iter()
        .map(|line| reg.replace_all(&line, "").to_string());
    unquoted_lines.flat_map(get_commands).collect()
}

pub(crate) type CommandMap = HashMap<String, (usize, Option<bool>, HashMap<String, usize>)>;

pub fn process_lines(lines: Vec<String>, _args: &Args) -> CommandMap {
    let leaders = ["sudo", "doas"];
    let super_commands = ["git", "entr", "time"];

    let mut output: CommandMap = HashMap::new();

    for line in lines.into_iter() {
        let words = line.split_whitespace().collect::<Vec<&str>>();

        let (first, second) = (words.first().unwrap().to_string(), words.get(1));

        output
            .entry(first.clone())
            .or_insert((0, None, HashMap::new()))
            .0 += 1;

        if let Some(second) = second {
            let mut parent_entry = output.get_mut(&first).unwrap();

            if parent_entry.1.is_some() {
                *parent_entry.2.entry(second.to_string()).or_insert(0) += 1;
            }

            if leaders.contains(&first.as_str()) {
                parent_entry.1 = Some(true);
                output
                    .entry((*second).to_string())
                    .or_insert((0, None, HashMap::new()))
                    .0 += 1;
            } else if super_commands.contains(&first.as_str()) {
                parent_entry.1 = Some(false);
            }
        }
    }
    output
}

#[cfg(test)]
mod parsing {
    #[test]
    #[ignore = "reformat pending ..."]
    fn tood() {
        todo!()
    }
}
