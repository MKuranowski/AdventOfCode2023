// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use aoc2023::day18::{Coords, Direction, PlanEntry, Trench};
use std::io::stdin;

fn load_input() -> Vec<PlanEntry> {
    stdin()
        .lines()
        .map(|line| {
            let line = line.expect("failed to read from stdin");
            let mut parts = line.split_ascii_whitespace();

            let dir = match parts.next().unwrap() {
                "U" => Direction::Up,
                "R" => Direction::Right,
                "D" => Direction::Down,
                "L" => Direction::Left,
                _ => panic!("invalid direction"),
            };

            let step = i32::from_str_radix(parts.next().unwrap(), 10).unwrap();

            PlanEntry { dir, step }
        })
        .collect()
}

fn main() {
    let plan = load_input();
    let trench = Trench::digged(&plan);

    let mut result: usize = 0;
    for x in trench.top..=trench.bottom {
        for y in trench.left..=trench.right {
            let pt = Coords(x, y);
            if pt.is_inside(&trench.corners) {
                result += 1;
            }
        }
    }

    eprintln!("{result}");
}
