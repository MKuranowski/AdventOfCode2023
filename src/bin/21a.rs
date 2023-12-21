// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use std::{collections::HashSet, io::stdin};

struct Map(Vec<Vec<u8>>);

impl Map {
    fn at(&self, pt: (i32, i32)) -> u8 {
        debug_assert!(pt.0 >= 0);
        debug_assert!(pt.1 >= 0);
        self.0[pt.0 as usize][pt.1 as usize]
    }

    fn rows(&self) -> i32 {
        self.0.len().try_into().unwrap()
    }

    fn columns(&self) -> i32 {
        self.0[0].len().try_into().unwrap()
    }

    fn neighbors(&self, pt: (i32, i32)) -> impl Iterator<Item = (i32, i32)> + '_ {
        [
            (pt.0 - 1, pt.1),
            (pt.0 + 1, pt.1),
            (pt.0, pt.1 - 1),
            (pt.0, pt.1 + 1),
        ]
        .into_iter()
        .filter(|(x, y)| {
            // why the fuck `|(&x, &y)|` is not possible is fucking beyond me
            let x: i32 = *x;
            let y: i32 = *y;

            x >= 0
                && x < self.rows()
                && y >= 0
                && y < self.columns()
                && self.0[x as usize][y as usize] != b'#'
        })
    }

    fn start(&self) -> (i32, i32) {
        for x in 0..self.rows() {
            for y in 0..self.columns() {
                if self.at((x, y)) == b'S' {
                    return (x, y);
                }
            }
        }
        panic!("No 'S' tile");
    }

    fn all_neighbors<I: IntoIterator<Item = (i32, i32)>>(&self, current: I) -> HashSet<(i32, i32)> {
        current
            .into_iter()
            .flat_map(|pt| self.neighbors(pt))
            .collect()
    }
}

fn load_input() -> Map {
    Map(stdin()
        .lines()
        .map(|l| l.expect("failed to read from stdin").into_bytes())
        .collect())
}

fn main() {
    let map = load_input();
    let mut points: HashSet<(i32, i32)> = HashSet::from_iter([map.start()]);

    for _ in 0..64 {
        points = map.all_neighbors(points);
    }

    println!("{}", points.len())
}
