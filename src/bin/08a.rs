// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use aoc2023::day08::load_input;

fn main() {
    let (moves, graph) = load_input();
    let mut at = *b"AAA";

    for (step, move_) in moves.iter().cycle().enumerate() {
        if &at == b"ZZZ" {
            println!("{step}");
            return
        }

        at = graph.get(&at).unwrap()[*move_ as usize];
    }
}
