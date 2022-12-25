use std::collections::HashMap;
use std::io::Read;

pub fn get_contents(path: String) -> String {
    let mut histfile_buffer = std::fs::File::open(path).unwrap();
    let mut contents = String::new();
    histfile_buffer.read_to_string(&mut contents).unwrap();

    contents
}

pub fn parse_contents(contents: String, prefix: Option<String>) -> HashMap<String, usize> {
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
