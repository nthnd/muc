use std::{collections::HashMap, env, fs};

fn main() {
    // if supplied with a number n as an argument, show the top n commands
    // otherwise show all
    let count = env::args()
        .collect::<Vec<String>>()
        .get(1)
        .map(|arg| arg.parse::<usize>().expect("Bad args"));

    // get the commands
    let commands = get_commands(&get_histfile_path());

    //print each command with its rank and frequency
    let (longest_num, longest_freq) = commands.first().unwrap();
    for (index, command) in commands.iter().enumerate() {
        println!(
            "{:indent_n$}: [{:indent_f$}] -> {}",
            index + 1,
            command.1,
            command.0,
            indent_n = longest_num.to_string().len(),
            indent_f = longest_freq.to_string().len(),
        );
        if let Some(x) = count {
            if index == x - 1 {
                break;
            }
        }
    }
}

fn get_histfile_path() -> String {
    // get the LOGNAME environment variable
    let login = env::var("LOGNAME").expect("couldnt find login name");

    // determine the histfile from the SHELL environment variable
    let histfile_path = match env::var("SHELL")
        .expect("couldnt find shell")
        .split('/')
        .last()
        .unwrap()
    {
        "zsh" => ".histfile",
        "bash" => ".bash_history",
        _ => ".alskdj",
    };

    //return the full path to the histfile
    format!("/home/{login}/{histfile_path}")
}

fn get_commands(path: &str) -> Vec<(String, usize)> {
    // iterate through the histfile's lines and get the commands
    let commands = fs::read_to_string(path)
        .expect("couldn't read file")
        .split(&['\n', '|', '&', ';'])
        .map(|x| x.to_owned())
        .filter(|x| !x.is_empty())
        .map(|x| {
            (*x.split_whitespace()
                .map(|x| x.to_owned())
                .collect::<Vec<String>>()
                .get(0)
                .unwrap())
            .clone()
        })
        .collect::<Vec<String>>();

    // count how many times the commands appear
    let mut frequencies: HashMap<String, usize> = HashMap::new();
    commands
        .iter()
        .for_each(|x| *frequencies.entry((*x).clone()).or_default() += 1);

    // sort the commands by frequency and return
    let mut sorted_frequencies = frequencies.into_iter().collect::<Vec<(String, usize)>>();
    sorted_frequencies.sort_by(|a, b| a.1.cmp(&b.1));
    sorted_frequencies
        .into_iter()
        .filter(|x| !x.0.starts_with("./"))
        .rev()
        .collect()
}
