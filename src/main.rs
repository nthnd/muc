use std::{collections::HashMap, env, fs};

fn main() {
    let count = env::args()
        .collect::<Vec<String>>()
        .get(1)
        .map(|arg| arg.parse::<usize>().expect("Bad args"));

    let commands = get_commands(&get_histfile_path());
    let len: f32 = commands.iter().map(|x| x.1).sum::<usize>() as f32;

    println!("Total invocations: {len}");

    let longest = commands.first().unwrap().1.to_string().len();
    for (index, command) in commands.iter().enumerate() {
        println!(
            "{:longest$} - {: >2}% - {}",
            command.1,
            ((command.1 as f32 / len) * 100.) as usize,
            command.0,
        );
        if let Some(x) = count {
            if index == x - 1 {
                break;
            }
        }
    }
}

fn get_histfile_path() -> String {
    let login = env::var("LOGNAME").expect("couldnt find login name");

    let histfile_path = match env::var("SHELL")
        .expect("couldnt find shell")
        .split('/')
        .last()
        .unwrap()
    {
        "zsh" => ".histfile",
        "bash" => ".bash_history",
        "fish" => ".local/share/fish/fish_history",
        _ => ".idek-Man",
    };

    format!("/home/{login}/{histfile_path}")
}

fn get_commands(path: &str) -> Vec<(String, usize)> {
    let _commands = fs::read_to_string(path)
        .expect("couldn't read file");

    let commands = match env::var("SHELL")
        .expect("couldnt find shell")
        .split('/')
        .last()
        .unwrap()
    {
        "fish" => {
          _commands.split(&['\n', '|', '&', ';'])
            .map(|x| x.to_owned())
            .filter(|x| !x.is_empty())
            .filter(|x| x.starts_with("- cmd: "))
            .map(|x| {
                match x.split_once("- cmd: ") {
                  Some((_, val)) => {
                    val.split_whitespace()
                      .map(|x| x.to_owned())
                      .collect::<Vec<String>>()
                      .first()
                      .unwrap()
                      .clone()
                  },
                  None => { panic!("Could not find commands in fish history") },
                }
            })
            .collect::<Vec<String>>()
        },
        _ => {
          _commands.split(&['\n', '|', '&', ';'])
            .map(|x| x.to_owned())
            .filter(|x| !x.is_empty())
            .map(|x| {
                x.split_whitespace()
                    .map(|x| x.to_owned())
                    .collect::<Vec<String>>()
                    .first()
                    .unwrap()
                    .clone()
            })
            .collect::<Vec<String>>()
        },
    };

    let mut frequencies: HashMap<String, usize> = HashMap::new();
    commands
        .iter()
        .for_each(|x| *frequencies.entry((*x).clone()).or_default() += 1);

    let mut sorted_frequencies = frequencies.into_iter().collect::<Vec<(String, usize)>>();
    sorted_frequencies.sort_by(|a, b| a.1.cmp(&b.1));
    sorted_frequencies
        .into_iter()
        .filter(|x| !x.0.starts_with("./"))
        .rev()
        .collect()
}
