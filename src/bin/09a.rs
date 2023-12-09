// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use aoc2023::day09::{extrapolate, load_input};

fn main() {
    let result = load_input().map(|l| extrapolate(l)).sum::<i32>();
    println!("{result}");
}
