// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::io::stdin;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coords(pub u16, pub u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    fn move_(self, from: Coords) -> Coords {
        match self {
            Self::N => Coords(from.0 - 1, from.1),
            Self::E => Coords(from.0, from.1 + 1),
            Self::S => Coords(from.0 + 1, from.1),
            Self::W => Coords(from.0, from.1 - 1),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pipe {
    Unknown,
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
}

impl Pipe {
    fn move_(self, from: Direction) -> Option<Direction> {
        match self {
            Pipe::Unknown => Some(from),
            Pipe::NS => match from {
                Direction::N | Direction::S => Some(from),
                _ => None,
            },
            Pipe::EW => match from {
                Direction::E | Direction::W => Some(from),
                _ => None,
            },
            Pipe::NE => match from {
                Direction::S => Some(Direction::E),
                Direction::W => Some(Direction::N),
                _ => None,
            },
            Pipe::NW => match from {
                Direction::E => Some(Direction::N),
                Direction::S => Some(Direction::W),
                _ => None,
            },
            Pipe::SW => match from {
                Direction::N => Some(Direction::W),
                Direction::E => Some(Direction::S),
                _ => None,
            },
            Pipe::SE => match from {
                Direction::N => Some(Direction::E),
                Direction::W => Some(Direction::S),
                _ => None,
            },
        }
    }
}

#[derive(Debug)]
pub struct Map {
    pub tiles: Vec<Vec<Option<Pipe>>>,
    pub start: Coords,
}

impl Map {
    pub fn move_(&self, at: Coords, dir: Direction) -> Option<(Coords, Direction)> {
        let next_at = dir.move_(at);
        let next_pipe = self.tiles[next_at.0 as usize][next_at.1 as usize];
        next_pipe
            .map(|pipe| pipe.move_(dir))
            .flatten()
            .map(|next_dir| (next_at, next_dir))
    }

    pub fn loops(&self, mut dir: Direction) -> bool {
        let mut at = self.start;

        if (at.0 == 0 && dir == Direction::N)
            || (at.1 == 0 && dir == Direction::W)
            || (at.0 as usize == self.tiles.len() - 1 && dir == Direction::S)
            || (at.1 as usize == self.tiles[0].len() - 1 && dir == Direction::E)
        {
            return false;
        }

        while let Some((next_at, next_dir)) = self.move_(at, dir) {
            if next_at == self.start {
                return true;
            }
            (at, dir) = (next_at, next_dir);
        }
        return false;
    }

    pub fn update_min_distances(&self, mut dir: Direction, distances: &mut HashMap<Coords, u32>) {
        let mut at = self.start;
        let mut steps: u32 = 0;
        while let Some((next_at, next_dir)) = self.move_(at, dir) {
            distances
                .entry(at)
                .and_modify(|current| *current = (*current).min(steps))
                .or_insert(steps);
            steps += 1;

            if next_at == self.start {
                return;
            }
            (at, dir) = (next_at, next_dir);
        }
        panic!("starting in {dir:?} did not loop");
    }

    pub fn update_all_min_distances(&self, distances: &mut HashMap<Coords, u32>) {
        [Direction::N, Direction::E, Direction::S, Direction::W]
            .into_iter()
            .filter(|d| self.loops(*d))
            .for_each(|d| self.update_min_distances(d, distances));
    }

    pub fn path(&self) -> PathIterator<'_> {
        let dir = [Direction::N, Direction::E, Direction::S, Direction::W]
            .into_iter()
            .filter(|d| self.loops(*d))
            .next()
            .unwrap();

        PathIterator::new(self, dir)
    }
}

pub struct PathIterator<'a> {
    map: &'a Map,
    at: Coords,
    dir: Direction,
    started: bool,
}

impl<'a> PathIterator<'a> {
    fn new(map: &'a Map, dir: Direction) -> Self {
        Self {
            map,
            at: map.start,
            dir,
            started: false,
        }
    }
}

impl<'a> Iterator for PathIterator<'a> {
    type Item = Coords;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.started {
            self.started = true;
            return Some(self.at);
        }

        (self.at, self.dir) = self
            .map
            .move_(self.at, self.dir)
            .expect("loop does not exist");

        if self.at == self.map.start {
            None
        } else {
            Some(self.at)
        }
    }
}

pub fn load_input() -> Map {
    let tiles: Vec<Vec<Option<Pipe>>> = stdin()
        .lines()
        .map(|line| {
            line.expect("failed to read from stdin")
                .as_bytes()
                .iter()
                .map(|c| match c {
                    b'|' => Some(Pipe::NS),
                    b'-' => Some(Pipe::EW),
                    b'L' => Some(Pipe::NE),
                    b'J' => Some(Pipe::NW),
                    b'7' => Some(Pipe::SW),
                    b'F' => Some(Pipe::SE),
                    b'S' => Some(Pipe::Unknown),
                    _ => None,
                })
                .collect()
        })
        .collect();

    let mut start = Coords(0, 0);
    for (x, row) in tiles.iter().enumerate() {
        for (y, pipe) in row.iter().enumerate() {
            if pipe == &Some(Pipe::Unknown) {
                start = Coords(x as u16, y as u16);
            }
        }
    }

    Map { tiles, start }
}
