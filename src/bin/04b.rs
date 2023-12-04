// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use aoc2023::day04::load_input;
use std::collections::HashMap;

fn main() {
    let mut counts: HashMap<u32, u32> = HashMap::default();
    for (card_id, expected, got) in load_input() {
        let count: u32 = {
            let count = counts.entry(card_id).or_insert(0);
            *count += 1; // Add the original card
            *count
        };

        let overlaps = expected.intersection(got).len() as u32;
        for new_card_offset in 1..=overlaps {
            let new_card_id = card_id + new_card_offset;
            let new_card_count = counts.entry(new_card_id).or_insert(0);
            *new_card_count += count;
        }
    }
    let result = counts.values().sum::<u32>();
    println!("{result}");
}
