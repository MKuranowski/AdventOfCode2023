// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use std::collections::HashMap;

use aoc2023::day10::{load_input, Coords};

fn main() {
    let map = load_input();
    let mut distances: HashMap<Coords, u32> = HashMap::default();
    map.update_all_min_distances(&mut distances);
    let result = distances.values().max().unwrap();
    println!("{result}");
}
