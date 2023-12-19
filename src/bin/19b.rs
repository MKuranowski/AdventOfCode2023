// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use aoc2023::day19::{load_input, PartRange};

fn main() {
    let (system, _) = load_input();
    let result = system.count_accepted(PartRange {
        x: 1..4001,
        m: 1..4001,
        a: 1..4001,
        s: 1..4001,
    });
    println!("{result}");
}
