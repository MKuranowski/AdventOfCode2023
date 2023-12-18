// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use aoc2023::day16::{count_energy_tiles, load_input, Map, DIR_DOWN, DIR_LEFT, DIR_RIGHT, DIR_UP};

fn top_edge_max(m: &Map) -> usize {
    (0..m.columns())
        .map(|y| count_energy_tiles(m, (0, y, DIR_DOWN)))
        .max()
        .unwrap()
}

fn left_edge_max(m: &Map) -> usize {
    (0..m.rows())
        .map(|x| count_energy_tiles(m, (x, 0, DIR_RIGHT)))
        .max()
        .unwrap()
}

fn bottom_edge_max(m: &Map) -> usize {
    let x = m.rows() - 1;
    (0..m.columns())
        .map(|y| count_energy_tiles(m, (x, y, DIR_UP)))
        .max()
        .unwrap()
}

fn right_edge_max(m: &Map) -> usize {
    let y = m.columns() - 1;
    (0..m.rows())
        .map(|x| count_energy_tiles(m, (x, y, DIR_LEFT)))
        .max()
        .unwrap()
}

fn all_max(m: &Map) -> usize {
    [
        top_edge_max(m),
        right_edge_max(m),
        bottom_edge_max(m),
        left_edge_max(m),
    ]
    .iter()
    .copied()
    .max()
    .unwrap()
}

fn main() {
    let map = load_input();
    let result = all_max(&map);
    println!("{result}");
}
