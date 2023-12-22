// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use std::collections::{HashMap, HashSet};
use std::io::stdin;
use std::ops::Range;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Cube(u16, u16, u16);

impl Cube {
    fn parse(x: &str) -> Self {
        let mut parts = x.split(',');
        let x = u16::from_str_radix(parts.next().unwrap(), 10).unwrap();
        let y = u16::from_str_radix(parts.next().unwrap(), 10).unwrap();
        let z = u16::from_str_radix(parts.next().unwrap(), 10).unwrap();
        Self(x, y, z)
    }

    fn range(l: Self, r: Self) -> Vec<Self> {
        let xs = l.0..=r.0;
        let ys = l.1..=r.1;
        let zs = l.2..=r.2;

        let mut result = Vec::with_capacity(xs.len() * ys.len() * zs.len());
        for x in xs {
            for y in ys.clone() {
                for z in zs.clone() {
                    result.push(Self(x, y, z));
                }
            }
        }
        result
    }

    fn parse_range(x: &str) -> Vec<Self> {
        let (l_str, r_str) = x.split_once('~').unwrap();
        let l = Self::parse(l_str);
        let r = Self::parse(r_str);
        Self::range(l, r)
    }
}

pub type BrickID = u16;

#[derive(Debug)]
struct Brick(Vec<Cube>);

#[derive(Debug)]
pub struct Bricks {
    by_id: Vec<Brick>,
    by_cube: HashMap<Cube, BrickID>,
}

impl Bricks {
    pub fn id_range(&self) -> Range<BrickID> {
        0..(self.by_id.len() as BrickID)
    }

    fn can_move_down(&self, id: usize) -> bool {
        for cube in &self.by_id[id].0 {
            let moved = Cube(cube.0, cube.1, cube.2 - 1);

            // Can't move into the floor
            if moved.2 == 0 {
                return false;
            }

            // Can't move onto another cube
            if let Some(&hit_id) = self.by_cube.get(&moved) {
                if id != hit_id as usize {
                    return false;
                }
            }
        }

        return true;
    }

    fn do_move_down(&mut self, id: usize) {
        // Remove from occupied
        for cube in &self.by_id[id].0 {
            self.by_cube.remove(cube);
        }

        // Move and set occupied
        for cube in &mut self.by_id[id].0 {
            cube.2 -= 1;
            self.by_cube.insert(*cube, id as BrickID);
        }
    }

    fn step_down_brick(&mut self, id: usize) -> bool {
        if self.can_move_down(id) {
            self.do_move_down(id);
            true
        } else {
            false
        }
    }

    fn step_down(&mut self) -> bool {
        let mut changed = false;
        for id in 0..self.by_id.len() {
            if self.step_down_brick(id) {
                changed = true;
            }
        }
        changed
    }

    pub fn all_down(&mut self) {
        while self.step_down() {}
    }

    fn foundations_of(&self, id: usize) -> HashSet<BrickID> {
        let mut foundations: HashSet<BrickID> = HashSet::default();

        for cube in &self.by_id[id].0 {
            let below = Cube(cube.0, cube.1, cube.2 - 1);
            if let Some(&cube_foundation) = self.by_cube.get(&below) {
                if id != cube_foundation as usize {
                    foundations.insert(cube_foundation);
                }
            }
        }

        foundations
    }

    pub fn safe_to_disintegrate(&self) -> HashSet<BrickID> {
        let mut safe_to_disintegrate: HashSet<BrickID> = (0..self.by_id.len() as BrickID).collect();

        for id in 0..self.by_id.len() {
            let foundations = self.foundations_of(id);
            if foundations.len() == 1 {
                for foundation_brick_id in foundations {
                    safe_to_disintegrate.remove(&foundation_brick_id);
                }
            }
        }

        safe_to_disintegrate
    }

    pub fn foundations(&self) -> HashMap<BrickID, HashSet<BrickID>> {
        self.id_range()
            .map(|i| (i, self.foundations_of(i as usize)))
            .collect()
    }
}

pub fn load_input() -> Bricks {
    let mut by_id = Vec::default();
    let mut by_cube = HashMap::default();

    for (idx, line) in stdin().lines().enumerate() {
        let line = line.expect("failed to read from stdin");
        let cubes = Cube::parse_range(&line);

        for cube in &cubes {
            by_cube.insert(*cube, idx as BrickID);
        }

        by_id.push(Brick(cubes));
    }

    Bricks { by_id, by_cube }
}
