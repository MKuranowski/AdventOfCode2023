// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use std::collections::{HashMap, VecDeque};
use std::io::stdin;
use std::iter::once;

use aoc2023::bitset::{BigBitset, SmallBitset};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
        }
    }
}

struct Map(Vec<Vec<u8>>);

impl Map {
    fn rows(&self) -> usize {
        self.0.len()
    }

    fn columns(&self) -> usize {
        self.0[0].len()
    }

    fn available_neighbor(
        &self,
        (x, y): (u8, u8),
        towards: Direction,
        came_from: Direction,
    ) -> Option<((u8, u8), Direction)> {
        let at_tile = self.0[x as usize][y as usize];

        // Tile and direction restrictions
        if at_tile == b'#'
            || (at_tile == b'>' && towards != Direction::Right)
            || (at_tile == b'v' && towards != Direction::Down)
            || came_from == towards
        {
            return None;
        }

        let (nx, ny) = match towards {
            Direction::Up => (x - 1, y),
            Direction::Right => (x, y + 1),
            Direction::Down => (x + 1, y),
            Direction::Left => (x, y - 1),
        };

        let new_tile = self.0[nx as usize][ny as usize];

        if new_tile != b'#' {
            Some(((nx, ny), towards))
        } else {
            None
        }
    }

    fn find_segments(&self) -> Vec<Segment> {
        // XXX: Except for the start and end points, the whole map is enclosed by a forest.
        //      This means that x - 1 or y - 1 will never overflow, except at those 2 points.
        let mut segments: Vec<Segment> = Vec::default();
        let mut visited_intersections = BigBitset::default();

        let mut q: VecDeque<(Segment, Direction)> = once(
            // XXX: The first step from start must be downwards, to prevent overflow
            //      we start the search on (1, 1).
            (
                Segment {
                    from: (0, 1),
                    to: (1, 1),
                    steps: 1,
                },
                Direction::Up,
            ),
        )
        .collect();

        while let Some((segment_so_far, came_from)) = q.pop_front() {
            if segment_so_far.to == (self.rows() as u8 - 1, self.columns() as u8 - 2) {
                // XXX: End reached. Can't follow normal code path as this risks overflows.
                segments.push(segment_so_far);
                continue;
            }

            let neighbors = [
                self.available_neighbor(segment_so_far.to, Direction::Up, came_from),
                self.available_neighbor(segment_so_far.to, Direction::Right, came_from),
                self.available_neighbor(segment_so_far.to, Direction::Down, came_from),
                self.available_neighbor(segment_so_far.to, Direction::Left, came_from),
            ];
            let neighbors_count = (&neighbors).iter().filter(|n| n.is_some()).count();

            let (new_segment_from, new_segment_steps) = if neighbors_count == 0 {
                // Dead end
                continue;
            } else if neighbors_count == 1 {
                // Not an intersection - continue with the current segment
                (segment_so_far.from, segment_so_far.steps + 1)
            } else {
                // Intersection - start a new section
                segments.push(segment_so_far);

                // Intersection already visited - don't re-expand it
                let to_compressed = compress(segment_so_far.to);
                if visited_intersections.contains(to_compressed) {
                    continue;
                }

                visited_intersections.insert(to_compressed);
                (segment_so_far.to, 1)
            };

            for (neighbor, towards) in neighbors.iter().filter_map(|&x| x) {
                q.push_back((
                    Segment {
                        from: new_segment_from,
                        to: neighbor,
                        steps: new_segment_steps,
                    },
                    towards.opposite(),
                ));
            }
        }

        segments
    }
}

fn compress((x, y): (u8, u8)) -> u64 {
    ((x as u64) << 8) | y as u64
}

fn load_input() -> Map {
    Map(stdin()
        .lines()
        .map(|l| l.expect("failed to read from stdin").into_bytes())
        .collect())
}

#[derive(Debug, Clone, Copy)]
struct Segment {
    from: (u8, u8),
    to: (u8, u8),
    steps: u16,
}

struct Search<'a> {
    segments: &'a [Segment],
    intersections: HashMap<(u8, u8), u8>,
    by_from: HashMap<(u8, u8), Vec<(u8, bool)>>,
}

impl<'a> Search<'a> {
    fn new(segments: &'a [Segment]) -> Self {
        let mut by_from: HashMap<(u8, u8), Vec<(u8, bool)>> = HashMap::default();
        let mut intersections: HashMap<(u8, u8), u8> = HashMap::default();
        let mut intersection_counter = 0_u8;

        for (idx, segment) in segments.iter().enumerate() {
            by_from
                .entry(segment.from)
                .or_default()
                .push((idx as u8, false));
            by_from
                .entry(segment.to)
                .or_default()
                .push((idx as u8, true));

            if !intersections.contains_key(&segment.from) {
                intersections.insert(segment.from, intersection_counter);
                intersection_counter += 1;
            }

            if !intersections.contains_key(&segment.to) {
                intersections.insert(segment.to, intersection_counter);
                intersection_counter += 1;
            }
        }
        Self {
            segments,
            intersections,
            by_from,
        }
    }

    fn run(&self, from: (u8, u8), to: (u8, u8)) -> u16 {
        let mut max_steps_to_node: HashMap<(u8, u8), u16> = HashMap::default();
        let mut q: VecDeque<((u8, u8), u16, SmallBitset, SmallBitset)> = once((
            from,
            0,
            SmallBitset::default(),
            bitset_with(self.intersections[&from]),
        ))
        .collect();

        while let Some((at, steps, visited_segments, visited_intersections)) = q.pop_front() {
            // Remember the visited set
            if let Some(known_steps_to) = max_steps_to_node.get_mut(&at) {
                *known_steps_to = (*known_steps_to).max(steps);
            } else {
                max_steps_to_node.insert(at, steps);
            }

            if let Some(neighbors) = self.by_from.get(&at) {
                for &(neighbor_id, is_reverse) in neighbors {
                    // Don't re-visit segments
                    if visited_segments.contains(neighbor_id as u32) {
                        continue;
                    }

                    let neighbor = &self.segments[neighbor_id as usize];

                    let destination = if is_reverse {
                        neighbor.from
                    } else {
                        neighbor.to
                    };

                    // Don't re-visit intersections
                    let dest_id = self.intersections[&destination];
                    if visited_intersections.contains(dest_id as u32) {
                        continue;
                    }

                    let mut new_visited_segments = visited_segments.clone();
                    new_visited_segments.insert(neighbor_id as u32);

                    let mut new_visited_intersections = visited_intersections.clone();
                    new_visited_intersections.insert(dest_id as u32);

                    q.push_back((
                        destination,
                        steps + neighbor.steps,
                        new_visited_segments,
                        new_visited_intersections,
                    ))
                }
            }
        }

        max_steps_to_node[&to]
    }
}

fn bitset_with(i: u8) -> SmallBitset {
    let mut s = SmallBitset::default();
    s.insert(i as u32);
    s
}

fn main() {
    let map = load_input();
    let segments = map.find_segments();
    let result =
        Search::new(&segments).run((0, 1), (map.rows() as u8 - 1, map.columns() as u8 - 2));
    println!("{result}");
}
