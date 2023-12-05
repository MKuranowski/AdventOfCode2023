// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use aoc2023::day05::load_input;

fn main() {
    let (seeds, almanac) = load_input();
    let result = seeds
        .iter()
        .map(|&i| almanac.get(i))
        .min()
        .unwrap();
    println!("{result}");
}
