// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use aoc2023::day19::load_input;

fn main() {
    let (system, parts) = load_input();
    let result = parts
        .iter()
        .filter(|&p| system.is_accepted(p))
        .map(|&p| p.sum())
        .sum::<usize>();
    println!("{result}");
}
