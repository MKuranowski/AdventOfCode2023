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

            parts.next().unwrap();
            parts.next().unwrap();
            let hex = parts.next().unwrap();

            let step = i32::from_str_radix(&hex[2..7], 16).unwrap();
            let dir = match hex.as_bytes()[7] {
                b'0' => Direction::Right,
                b'1' => Direction::Down,
                b'2' => Direction::Left,
                b'3' => Direction::Up,
                _ => panic!("Invalid direction"),
            };

            PlanEntry { dir, step }
        })
        .collect()
}

fn space_partitions<const DIM: u32>(corners: &[Coords]) -> Vec<i32> {
    let f: fn(&Coords) -> i32 = if DIM == 0 { |x| x.0 } else { |x| x.1 };
    let mut xs: Vec<i32> = corners.iter().map(f).collect();
    xs.sort();
    xs.dedup();
    xs.push(*xs.last().unwrap() + 1); // For windows to work correctly
    xs
}

fn main() {
    let plan = load_input();
    let trench = Trench::digged(&plan);

    let xs = space_partitions::<0>(&trench.corners);
    let ys = space_partitions::<1>(&trench.corners);
    let mut result: usize = 0;

    for horizontal in xs.windows(2) {
        let top = horizontal[0];
        let bottom = horizontal[1];
        for vertical in ys.windows(2) {
            let left = vertical[0];
            let right = vertical[1];

            // Check if the top-left corner is contained
            if Coords(top, left).is_inside(&trench.corners) {
                result += 1;
            }

            // Check if the top edge is contained
            let top_len = right - left - 1;
            if top_len > 0 && Coords(top, left + 1).is_inside(&trench.corners) {
                result += top_len as usize;
            }

            // Check if left edge is contained
            let left_len = bottom - top - 1;
            if left_len > 0 && Coords(top + 1, left).is_inside(&trench.corners) {
                result += left_len as usize;
            }

            // Check if the insides are contained
            if top_len > 0 && left_len > 0 && Coords(top + 1, left + 1).is_inside(&trench.corners) {
                result += top_len as usize * left_len as usize;
            }
        }
    }

    eprintln!("{result}");
}
