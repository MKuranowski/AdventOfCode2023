// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use aoc2023::day11::{expand, load_input, sum_distances};

fn main() {
    let mut galaxies = load_input();
    // NOTE: "Replaced by 1 million rows" <=> "Expanded by 999 999 rows"
    expand(&mut galaxies, 999_999);
    let result = sum_distances(&galaxies);
    println!("{result}");
}
