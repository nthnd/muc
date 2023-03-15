use crossterm::{
    execute, queue,
    style::{Color, Print, PrintStyledContent, ResetColor, SetForegroundColor, Stylize},
};
use std::{
    collections::{BTreeMap, HashMap},
    io::{stdout, Stdout, Write},
    usize,
};
use utf8_slice::slice;

use crate::{hist_file::CommandMap, Args};

type VeryComplexType = (String, Option<bool>, HashMap<String, usize>);
pub fn print(data: CommandMap, args: Args) {
    let tree: BTreeMap<usize, VeryComplexType> = data
        .into_iter()
        .map(|(s, (f, o, h))| (f, (s, o, h)))
        .collect();

    let total: usize = tree.keys().sum();
    if total == 0 {
        println!("No commands found");
        return;
    }
    let max = *tree.last_key_value().unwrap().0;

    let reversed_tree: Vec<(usize, VeryComplexType)> = tree.into_iter().rev().collect();
    let limited_tree = reversed_tree[..(usize::min(args.count, reversed_tree.len()))].to_vec();

    let mut stdout = stdout();

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

        print_command(s, *freq, max, total, &args, sub_commands, &mut stdout);
    }

    stdout.flush().unwrap();

    let others = total - limited_tree.iter().fold(0, |acc, x| acc + x.0);
    let other_percentage = (others as f64 / total as f64) * 100.;
    execute! {
        stdout,
        SetForegroundColor(Color::Grey),
        Print(format!("...{} ({:.2}%) others\n", others, other_percentage)),
        ResetColor,
    }
    .unwrap();
    execute! {
        stdout,
        Print(format!("Total: {} commands\n", total))
    }
    .unwrap();
}

pub fn print_command(
    command: &str,
    invocations: usize,
    max: usize,
    total: usize,
    args: &Args,
    sub_commands: Option<Vec<String>>,
    stdout: &mut Stdout,
) {
    let percentage = (invocations as f32 / total as f32) * 100.0;
    let num_of_bars = ((invocations as f32 / max as f32) * 10.) as usize;
    let bar: String = format!(
        "{}{}",
        args.bar.fill.repeat(num_of_bars),
        args.bar.empty.repeat(10 - num_of_bars)
    );
    let pretty_sub_commands = if let Some(sub_commands) = sub_commands {
        format!("{} ...", sub_commands[..(sub_commands.len().min(3))].join(", "))
    } else {
        "".to_string()
    };

    queue!(
        stdout,
        SetForegroundColor(Color::Reset),
        Print(&args.bar.opening),
        PrintStyledContent(format!("{: <2}", slice(&bar, 0, 2)).red()),
        PrintStyledContent(format!("{: <3}", slice(&bar, 2, 5)).yellow()),
        PrintStyledContent(format!("{: <5}", slice(&bar, 5, 10)).green()),
        PrintStyledContent(format!("{} ", args.bar.closing).reset()),
        Print(format!("{: >5.2}% ", percentage)),
        PrintStyledContent(format!("{:5}", invocations).grey()),
        PrintStyledContent(format!(" {} ", command).reset().bold()),
        PrintStyledContent(format!("{}\n", pretty_sub_commands).reset().grey()),
        SetForegroundColor(Color::Reset),
    )
    .unwrap();
}
