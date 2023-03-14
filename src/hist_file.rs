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

fn get_commands(line: String) -> Vec<String> {
    line.split(&['&', '|', ';'])
        .filter(|x| !x.is_empty())
        .map(str::to_string)
        .collect()
}

/// Takes contents of a file and returns a vector of valid commands
pub fn parse_contents(contents: String, args: &Args) -> Vec<String> {
    let lines = contents
        .lines()
        .filter(|line| !line.is_empty())
        .map(str::trim);

    fn bash_strat(line: &str) -> &str {
        line
    }

    fn fish_strat(line: &str) -> &str {
        if line.starts_with("when: ") {
            ""
        } else {
            &line[7..]
        }
    }

    fn ohmyzsh_strat(line: &str) -> &str {
        &line[7..]
    }

    let shell_parsed = lines.map(match args.shell.as_str() {
        "fish" => fish_strat,
        "ohmyzsh" => ohmyzsh_strat,
        _ => bash_strat,
    });

    let reg = Regex::new("('(?:.|[^'\n])*'|\"(?:.|[^\"\n])*\")").unwrap();

    let unquoted_lines = shell_parsed.map(|line| reg.replace_all(line, "").to_string());
    let command_lines = unquoted_lines
        .flat_map(get_commands)
        .collect();

    command_lines
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
                    .entry(( *second ).to_string())
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
