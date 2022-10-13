use std::{collections::HashMap, env, fs};

fn main() {
    let args = env::args().skip(1).collect::<Vec<String>>();

    let arg = args.get(0);

    let count = if arg.is_none() {
        None
    } else {
        Some(arg.unwrap().parse::<usize>().expect("bad args"))
    };

    let sorted_f = get_commands("/home/nate/.histfile");
    let largest_num = sorted_f.get(0).unwrap().1.to_string().len();
    let largest_freq = sorted_f.get(0).unwrap().0.to_string().len();

    for (index, command) in sorted_f.iter().enumerate() {
        println!(
            "{:indent_n$}: [{:indent_f$}] -> {}",
            index + 1,
            command.1,
            command.0,
            indent_n = largest_num,
            indent_f = largest_freq + 2,
        );

        if let Some(x) = count {
            if index == x - 1 {
                break;
            }
        }
    }
}

fn get_commands(path: &str) -> Vec<(String, usize)> {
    let contents = fs::read_to_string(path).expect("couldn't read file");

    let lines: Vec<String> = contents
        .split(&['\n', '|', '&', ';'])
        .map(|x| x.to_owned())
        .filter(|x| !x.is_empty())
        .collect::<Vec<String>>();

    let commands: Vec<String> = lines
        .iter()
        .map(|x| {
            (*x.split_whitespace()
                .map(|x| x.to_owned())
                .collect::<Vec<String>>()
                .get(0)
                .unwrap())
            .clone()
        })
        .collect::<Vec<String>>();

    let mut frequencies: HashMap<String, usize> = HashMap::new();

    commands
        .iter()
        .for_each(|x| *frequencies.entry((*x).clone()).or_default() += 1);

    let mut sorted_f = frequencies.into_iter().collect::<Vec<(String, usize)>>();
    sorted_f.sort_by(|a, b| a.1.cmp(&b.1));
    sorted_f = sorted_f
        .into_iter()
        .filter(|x| !x.0.starts_with("./"))
        .rev()
        .collect();

    sorted_f
}
