// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use aoc2023::day15::{load_input, run_hash};

fn main() {
    let result = load_input()
        .split(',')
        .map(|step| run_hash(step.as_bytes()) as u32)
        .sum::<u32>();
    println!("{result}")
}
