// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use std::io::*;
use aoc2023::day02::*;

pub fn main() {
    let mut result: u32 = 0;

    for line in stdin().lines() {
        let line = line.unwrap();
        let (_, bags) = parse_game(&line);
        let game_set = bags.reduce(|previous, current| previous.max(&current)).unwrap();
        result += game_set.power();
    }

    println!("{result}");
}
