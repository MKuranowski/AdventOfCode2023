// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use aoc2023::day13::{find_reflection_line, load_input};

fn main() {
    let result = load_input()
        .iter()
        .map(|img| find_reflection_line(img))
        .sum::<usize>();
    println!("{result}");
}
