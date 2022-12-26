use std::{collections::HashMap, usize};
use utf8_slice::slice;

use aecir::style::{Color, ColorName, Format};

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
    let bar: String = format!(
        "{: <10}",
        "▮".repeat(((invocations as f32 / max as f32) * 10.) as usize)
    );
    println!(
        "[{red}{bar_first: <2}{yellow}{bar_second: <3}{green}{bar_third: <5}{reset}] \
        {percentage: >2}% {gray}{invocations:command_indentation$}{reset}\
        {bold} {command} {reset_style}",
        red = Color::Fg(ColorName::Red),
        bar_first = slice(&bar, 0, 2),
        yellow = Color::Fg(ColorName::Yellow),
        bar_second = slice(&bar, 2, 5),
        green = Color::Fg(ColorName::Green),
        bar_third = slice(&bar, 5, 10),
        reset = aecir::style::reset_colors(),
        gray = Color::Fg(ColorName::LightBlack),
        bold = Format::Bold,
        reset_style = aecir::style::reset_all(),
    );
}
