use std::{
    collections::{BTreeMap, HashMap},
    usize,
};
use utf8_slice::slice;

use crate::{hist_file::CommandMap, Args};
use aecir::style::{Color, ColorName, Format};

type VeryComplexType = (String, Option<bool>, HashMap<String, usize>);
pub fn print(data: CommandMap, args: Args) {
    let tree: BTreeMap<usize, VeryComplexType> = data
        .into_iter()
        .map(|(s, (f, o, h))| (f, (s, o, h)))
        .collect();

    let total: usize = tree.keys().sum();
    let max = *tree.last_key_value().unwrap().0;

    let reversed_tree: Vec<(usize, VeryComplexType)> = tree.into_iter().rev().collect();
    let limited_tree = reversed_tree[..(usize::min(args.count, reversed_tree.len()))].to_vec();

    for (freq, elem) in limited_tree.iter() {
        let (s, _o, h) = elem;
        let mut sub_commands = h.iter().collect::<Vec<(&String, &usize)>>();
        sub_commands.sort_by(|a, b| b.1.cmp(a.1));

        let sub_commands = if sub_commands.is_empty() {
            None
        } else {
            Some(
                sub_commands[..(usize::min(3, sub_commands.len()))]
                    .iter()
                    .map(|x| x.0.to_owned())
                    .collect(),
            )
        };

        print_command(s, *freq, max, total, &args, sub_commands);
    }

    let others = total - limited_tree.iter().fold(0, |acc, x| acc + x.0);
    let other_percentage = (others as f64 / total as f64) * 100.;
    println!(
        "{gray}... {} ({:.2}%) others{reset}",
        others,
        other_percentage,
        reset = aecir::style::reset_all(),
        gray = Color::Fg(ColorName::LightBlack),
    );
    println!(
        "Total: {} commands" ,
        total
    );
}

pub fn print_command(
    command: &str,
    invocations: usize,
    max: usize,
    total: usize,
    args: &Args,
    sub_commands: Option<Vec<String>>,
) {
    let percentage = (invocations as f32 / total as f32) * 100.0;
    let num_of_bars = ((invocations as f32 / max as f32) * 10.) as usize;
    let bar: String = format!(
        "{}{}",
        args.bar.fill.repeat(num_of_bars),
        args.bar.empty.repeat(10 - num_of_bars)
    );
    let pretty_sub_commands = if let Some(sub_commands) = sub_commands {
        let trim_len = sub_commands.len().min(3);
        let mut x = sub_commands[..trim_len].join(", ");
        x.push_str(" ...");
        x
    } else {
        "".to_string()
    };

    let opening_char = &args.bar.opening;
    let bar_first = slice(&bar, 0, 2);
    let bar_second = slice(&bar, 2, 5);
    let bar_third = slice(&bar, 5, 10);
    let closing_char = &args.bar.closing;

    let reset_style = aecir::style::reset_all();

    let (red, yellow, green, gray, bold) = (
        Color::Fg(ColorName::Red).to_string(),
        Color::Fg(ColorName::Yellow).to_string(),
        Color::Fg(ColorName::Green).to_string(),
        Color::Fg(ColorName::LightBlack).to_string(),
        Format::Bold.to_string(),
    );

    println!(
        "{opening_char}{red}{bar_first: <2}{yellow}{bar_second: <3}{green}{bar_third: <5}{reset_style}{closing_char} \
        {percentage: >5.2}% {gray}{invocations:5}{reset_style}\
        {bold} {command} {reset_style}{gray}{pretty_sub_commands} {reset_style}",
    );
}
