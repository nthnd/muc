use std::{
    collections::{BTreeMap, HashMap},
    usize,
};
use utf8_slice::slice;

use crate::{hist_file::CommandMap, Args};
use aecir::style::{Color, ColorName, Format};

type VeryComplexType = (String, Option<bool>, HashMap<String, usize>);
pub fn display_sorted(data: CommandMap, args: Args) {
    let tree: BTreeMap<usize, VeryComplexType> = data
        .into_iter()
        .map(|(s, (f, o, h))| (f, (s, o, h)))
        .collect();

    let total = tree.len();
    let max = *tree.last_key_value().unwrap().0;

    let limited_tree: Vec<(usize, VeryComplexType)> =
        tree.into_iter().rev().collect();

    for (freq, elem) in
        limited_tree[..(usize::min(args.count.unwrap(), limited_tree.len()))].iter()
    {
        let (s, _o, h) = elem;
        let mut sub_commands = h.iter().collect::<Vec<(&String, &usize)>>();
        sub_commands.sort_by(|a, b| b.1.cmp(a.1));
        let sub_commands = sub_commands[..(usize::min(3, sub_commands.len()))]
            .iter()
            .map(|x| x.0.to_owned())
            .collect();

        print_command(
            s,
            *freq,
            *freq as f32 / total as f32,
            max,
            &args,
            Some(sub_commands),
        );
    }
}

pub fn print_command(
    command: &str,
    invocations: usize,
    percentage: f32,
    max: usize,
    args: &Args,
    sub_commands: Option<Vec<String>>,
) {
    let bar: String = format!(
        "{: <10}",
        args.bar
            .to_string()
            .repeat(((invocations as f32 / max as f32) * 10.) as usize)
    );
    let pretty_sub_commands = if let Some(sub_commands) = sub_commands {
        let trim_len = sub_commands.len().min(3);
        let mut x = sub_commands[..trim_len].join(", ");
        x.push_str(" ...");
        x
    } else {
        "".to_string()
    };
    if args.pretty {
        println!(
            "{opening_char}{red}{bar_first: <2}{yellow}{bar_second: <3}{green}{bar_third: <5}{reset}{closing_char} \
            {percentage: >5.2}% {gray}{invocations:5}{reset}\
            {bold} {command} {reset_style}{gray}{pretty_sub_commands} {reset}",
            opening_char = args.bar_open,
            red = Color::Fg(ColorName::Red),
            bar_first = slice(&bar, 0, 2),
            yellow = Color::Fg(ColorName::Yellow),
            bar_second = slice(&bar, 2, 5),
            green = Color::Fg(ColorName::Green),
            bar_third = slice(&bar, 5, 10),
            reset = aecir::style::reset_colors(),
            closing_char = args.bar_close,
            gray = Color::Fg(ColorName::LightBlack),
            bold = Format::Bold,
            reset_style = aecir::style::reset_all(),
        );
    } else {
        println!(
            "{opening_char}{bar}{closing_char} {percentage: >5.2}% {invocations:5} {command}",
            opening_char = args.bar_open,
            closing_char = args.bar_close,
        );
    }
}
