// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use aoc2023::day11::{expand, load_input, sum_distances};

fn main() {
    let mut galaxies = load_input();
    expand(&mut galaxies, 1);
    let result = sum_distances(&galaxies);
    println!("{result}");
}
