// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use std::io::*;
use aoc2023::day02::*;

pub fn main() {
    let expected_bag = Bag { red: 12, green: 13, blue: 14 };
    let mut result: u32 = 0;

    for line in stdin().lines() {
        let line = line.unwrap();
        let (game_id, mut bags) = parse_game(&line);
        let is_possible = bags.all(|bag| bag.is_subset_of(&expected_bag));

        if is_possible {
            result += game_id;
        }
    }

    println!("{result}");
}
