// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use aoc2023::day17::{load_input, Search};

fn main() {
    let map = load_input();
    let result = Search::new(&map, 0, 3).run();
    println!("{result}");
}
