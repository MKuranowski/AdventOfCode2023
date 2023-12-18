// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::io::stdin;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn next_steps(self) -> [Self; 3] {
        match self {
            Self::Up => [Self::Left, Self::Up, Self::Right],
            Self::Right => [Self::Up, Self::Right, Self::Down],
            Self::Down => [Self::Right, Self::Down, Self::Left],
            Self::Left => [Self::Down, Self::Left, Self::Up],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coords(pub i16, pub i16);

impl Coords {
    pub fn moved(self, dir: Direction) -> Self {
        match dir {
            Direction::Up => Coords(self.0 - 1, self.1),
            Direction::Right => Coords(self.0, self.1 + 1),
            Direction::Down => Coords(self.0 + 1, self.1),
            Direction::Left => Coords(self.0, self.1 - 1),
        }
    }

    pub fn dist(self, other: Self) -> u16 {
        (other.0 - self.0).abs() as u16 + (other.1 - self.1) as u16
    }
}

#[derive(Default, Debug)]
pub struct Map(Vec<Vec<u8>>);

impl Map {
    pub fn rows(&self) -> i16 {
        self.0.len() as i16
    }

    pub fn columns(&self) -> i16 {
        self.0[0].len() as i16
    }

    pub fn in_bounds(&self, pt: Coords) -> bool {
        pt.0 >= 0 && pt.0 < self.rows() && pt.1 >= 0 && pt.1 < self.columns()
    }

    pub fn at(&self, pt: Coords) -> u16 {
        self.0[pt.0 as usize][pt.1 as usize] as u16
    }
}

fn digit_to_value(x: &mut u8) {
    if x.is_ascii_digit() {
        *x -= b'0';
    } else {
        panic!("invalid digit");
    }
}

pub fn load_input() -> Map {
    Map(stdin()
        .lines()
        .map(|line| {
            let mut line = line.expect("failed to read from stdin").into_bytes();
            line.iter_mut().for_each(digit_to_value);
            line
        })
        .collect())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct SearchNode {
    coords: Coords,
    dir: Direction,
    steps: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SearchQueueEntry {
    node: SearchNode,
    cost: u16,
    score: u16,
}

impl PartialOrd for SearchQueueEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SearchQueueEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        // NOTE: Rust's BinaryHeap is max-heap, while A* requires a min-heap - hence the reverse.
        self.score.cmp(&other.score).reverse()
    }
}

#[derive(Debug)]
pub struct Search<'a> {
    map: &'a Map,
    known_costs: HashMap<SearchNode, u16>,
    queue: BinaryHeap<SearchQueueEntry>,
    end: Coords,
    min_steps: u8,
    max_steps: u8,
}

impl<'a> Search<'a> {
    pub fn new(map: &'a Map, min_steps: u8, max_steps: u8) -> Self {
        Self {
            map: map,
            known_costs: HashMap::default(),
            queue: BinaryHeap::default(),
            end: Coords(map.rows() - 1, map.columns() - 1),
            min_steps,
            max_steps,
        }
    }

    fn add_initial_entry(&mut self, dir: Direction) {
        let node = SearchNode {
            coords: Coords(0, 0),
            dir,
            steps: 0,
        };

        let entry = SearchQueueEntry {
            node,
            cost: 0,
            score: Coords(0, 0).dist(self.end),
        };
        self.known_costs.insert(node, 0);
        self.queue.push(entry);
    }

    fn add_initial_entries(&mut self) {
        self.add_initial_entry(Direction::Right);
        self.add_initial_entry(Direction::Down);
    }

    pub fn run(&mut self) -> u16 {
        self.add_initial_entries();

        while let Some(entry) = self.queue.pop() {
            // End reached
            if entry.node.coords == self.end {
                return entry.cost;
            }

            // Add neighbors to the queue
            for next_dir in entry.node.dir.next_steps() {
                let next_coords = entry.node.coords.moved(next_dir);

                // Don't go beyond the map
                if !self.map.in_bounds(next_coords) {
                    continue;
                }

                // Check if the step conforms to the requirements
                if (entry.node.dir == next_dir && entry.node.steps >= self.max_steps)
                    || (entry.node.dir != next_dir && entry.node.steps < self.min_steps)
                {
                    continue;
                }

                // Prepare the node
                let next_node = SearchNode {
                    coords: next_coords,
                    dir: next_dir,
                    steps: if entry.node.dir == next_dir {
                        entry.node.steps + 1
                    } else {
                        1
                    },
                };
                let next_cost = entry.cost + self.map.at(next_coords);

                // Check if a cheaper route is known
                let known_cost = self.known_costs.entry(next_node).or_insert(u16::MAX);
                if next_cost >= *known_cost {
                    continue;
                }

                // Push to queue
                *known_cost = next_cost;
                self.queue.push(SearchQueueEntry {
                    node: next_node,
                    cost: next_cost,
                    score: next_cost + next_coords.dist(self.end),
                });
            }
        }

        panic!("No route found :^(");
    }
}
