use std::{collections::HashMap, usize};

pub fn display_sorted(data: HashMap<String, usize>, count: Option<usize>) {
    let mut sorted: Vec<(String, usize)> = data.into_iter().collect::<Vec<(String, usize)>>();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));

    let total = sorted.iter().fold(0, |acc, x| acc + x.1);
    let max = sorted[0].1;
    let command_indentation = max.to_string().len();

    let limited_array = if let Some(limit) = count {
        &sorted[0..limit]
    } else {
        &sorted
    };

    for item in limited_array {
        print_command(
            &item.0,
            item.1,
            item.1 * 100 / total,
            max,
            command_indentation,
        );
    }

}

pub fn print_command(
    command: &str,
    invocations: usize,
    percentage: usize,
    max: usize,
    command_indentation: usize,
) {
    println!(
        "[{: <10}] {: <2}% {:command_indentation$} = {}",
        "▮".repeat(((invocations as f32 / max as f32) * 10.) as usize),
        percentage,
        invocations,
        command
    );
}
