// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use aoc2023::day04::{calc_score, load_input};

fn main() {
    let result = load_input()
        .map(|(_, expected, got)| calc_score(expected, got))
        .sum::<u32>();
    println!("{result}");
}
