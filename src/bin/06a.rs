// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use aoc2023::day06::Race;
use std::io::stdin;

pub fn load_input() -> Vec<Race> {
    let mut stdin_lines = stdin()
        .lines()
        .map(|r| r.expect("failed to read from stdin"));
    let times_line = stdin_lines.next().unwrap();
    let distances_line = stdin_lines.next().unwrap();

    let times = times_line
        .split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|i| usize::from_str_radix(i, 10).unwrap());

    let distances = distances_line
        .split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|i| usize::from_str_radix(i, 10).unwrap());

    times
        .zip(distances)
        .map(|(time, distance)| Race { time, distance })
        .collect()
}

fn main() {
    let races = load_input();
    let result = races
        .iter()
        .map(|r| r.winning_range().len() as u64)
        .product::<u64>();
    println!("{result}");
}
