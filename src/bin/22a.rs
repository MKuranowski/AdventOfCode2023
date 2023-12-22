// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use aoc2023::day22::load_input;

fn main() {
    let mut bricks = load_input();
    bricks.all_down();
    let result = bricks.safe_to_disintegrate().len();
    eprintln!("{result}");
}
