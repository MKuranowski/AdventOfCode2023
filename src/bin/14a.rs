// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use aoc2023::day14::load_input;

fn main() {
    let mut platform = load_input();
    platform.tilt_north();

    let result = platform.north_load();
    println!("{result}")
}
