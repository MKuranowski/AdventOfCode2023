// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use aoc2023::day16::{count_energy_tiles, load_input, DIR_RIGHT};

fn main() {
    let map = load_input();
    let result = count_energy_tiles(&map, (0, 0, DIR_RIGHT));
    println!("{result}");
}
