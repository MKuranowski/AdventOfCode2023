// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use aoc2023::day06::Race;
use std::io::stdin;

fn parse_single_usize_ignoring_non_digits(mut l: String) -> usize {
    l.retain(|c| c.is_ascii_digit());
    usize::from_str_radix(&l, 10).unwrap()
}

pub fn load_input() -> Race {
    let mut stdin_lines = stdin()
        .lines()
        .map(|r| r.expect("failed to read from stdin"));

    let time = parse_single_usize_ignoring_non_digits(stdin_lines.next().unwrap());
    let distance = parse_single_usize_ignoring_non_digits(stdin_lines.next().unwrap());
    Race { time, distance }
}

fn main() {
    let race = load_input();
    let result = race.winning_range().len();
    println!("{result}");
}
