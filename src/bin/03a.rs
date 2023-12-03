// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use aoc2023::day03::load_input;

fn main() {
    let (numbers, points) = load_input();
    let result: u32 = numbers
        .iter()
        .filter(|&number| number.adjacent().any(|pt| points.contains_key(&pt)))
        .map(|number| number.value)
        .sum();
    println!("{result}");
}
