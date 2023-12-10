// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use std::collections::HashSet;

use aoc2023::day10::{load_input, Coords};

fn is_enclosed(path: &[Coords], pt: Coords) -> bool {
    // https://en.wikipedia.org/wiki/Even%E2%80%93odd_rule#Implementation
    let mut j = path.len() - 1;
    let mut c = false;

    for i in 0..path.len() {
        if pt == path[i] {
            return true;
        }

        if (path[i].1 > pt.1) != (path[j].1 > pt.1) {
            let slope = (pt.0 as i32 - path[i].0 as i32) * (path[j].1 as i32 - path[i].1 as i32)
                - (path[j].0 as i32 - path[i].0 as i32) * (pt.1 as i32 - path[i].1 as i32);

            if slope == 0 {
                return true;
            }

            if (slope < 0) != (path[j].1 < path[i].1) {
                c = !c;
            }
        }

        j = i;
    }

    return c;
}

fn main() {
    let map = load_input();
    let path: Vec<Coords> = map.path().collect();
    let path_set: HashSet<Coords> = path.iter().copied().collect();

    let mut count: u32 = 0;
    for x in 0..map.tiles.len() {
        for y in 0..map.tiles[0].len() {
            let coords = Coords(x as u16, y as u16);
            if !path_set.contains(&coords) && is_enclosed(&path, coords) {
                count += 1;
            }
        }
    }

    println!("{count}");
}
