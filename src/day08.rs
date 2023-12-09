// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::io::stdin;

#[derive(Debug, Copy, Clone)]
pub enum Move {
    Left = 0,
    Right = 1,
}

pub type Node = [u8; 3];
pub type Graph = HashMap<Node, [Node; 2]>;

pub fn load_input() -> (Vec<Move>, Graph) {
    let mut lines = stdin()
        .lines()
        .map(|l| l.expect("failed to read from stdin"));

    let moves_line = lines.next().unwrap();
    let moves = moves_line
        .as_bytes()
        .iter()
        .map(|&c| match c {
            b'L' => Move::Left,
            b'R' => Move::Right,
            _ => panic!("invalid move {}", c as char),
        })
        .collect();

    lines.next().unwrap(); // Skip blank line

    let graph = lines
        .map(|line| {
            let line = line.as_bytes();
            let start = [line[0], line[1], line[2]];
            let left = [line[7], line[8], line[9]];
            let right = [line[12], line[13], line[14]];
            (start, [left, right])
        })
        .collect();

    (moves, graph)
}
