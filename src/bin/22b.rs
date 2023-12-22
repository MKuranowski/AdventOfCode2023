// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use std::collections::{HashMap, HashSet};
use std::iter::once;

use aoc2023::day22::{load_input, BrickID};

fn chain_reaction_len_from_removing(
    foundations: &HashMap<BrickID, HashSet<BrickID>>,
    root: BrickID,
) -> usize {
    let mut removed: HashSet<BrickID> = once(root).collect();
    let mut last_removed_len = removed.len();

    loop {
        for (&brick_id, foundations) in foundations.iter() {
            if removed.contains(&brick_id) {
                continue;
            }

            // No foundations - laying on the floor, can't be removed
            if foundations.is_empty() {
                continue;
            }

            // Foundations of a brick are all removed - this brick also breaks
            if foundations.is_subset(&removed) {
                removed.insert(brick_id);
            }
        }

        // Removed set is no longer growing - simulation over
        if removed.len() == last_removed_len {
            return removed.len() - 1;
        } else {
            last_removed_len = removed.len();
        }
    }
}

fn main() {
    let mut bricks = load_input();
    bricks.all_down();

    let foundations = bricks.foundations();

    let result = bricks
        .id_range()
        .map(|id| chain_reaction_len_from_removing(&foundations, id))
        .sum::<usize>();

    println!("{result}");
}
