// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use aoc2023::day08::{load_input, Graph, Move, Node};
use num::Integer;

fn navigate(from: Node, moves: &[Move], graph: &Graph) -> usize {
    let mut at = from;
    for (step, move_) in moves.iter().cycle().enumerate() {
        if at[2] == b'Z' {
            return step;
        }
        at = graph.get(&at).unwrap()[*move_ as usize];
    }
    panic!("no path from {from:?}");
}

fn starting_nodes<'a>(graph: &'a Graph) -> impl Iterator<Item = &Node> + 'a {
    graph
        .keys()
        .filter(|node| node[2] == b'A')
}

fn main() {
    let (moves, graph) = load_input();
    let result = starting_nodes(&graph)
        .map(|&node| navigate(node, &moves, &graph))
        .fold(1_usize, |a, b| a.lcm(&b));
    println!("{result}");
}
