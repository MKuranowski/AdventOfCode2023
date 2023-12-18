// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::io::stdin;

pub type Direction = u8;
pub const DIR_UP: Direction = 1 << 0;
pub const DIR_RIGHT: Direction = 1 << 1;
pub const DIR_DOWN: Direction = 1 << 2;
pub const DIR_LEFT: Direction = 1 << 3;

#[derive(Default, Debug)]
pub struct Map(pub Vec<Vec<u8>>);

impl Map {
    #[inline]
    pub fn rows(&self) -> i16 {
        self.0.len() as i16
    }

    #[inline]
    pub fn columns(&self) -> i16 {
        self.0[0].len() as i16
    }
}

pub fn apply_dir(x: i16, y: i16, dir: Direction) -> (i16, i16, Direction) {
    if dir & DIR_UP != 0 {
        (x - 1, y, dir)
    } else if dir & DIR_RIGHT != 0 {
        (x, y + 1, dir)
    } else if dir & DIR_DOWN != 0 {
        (x + 1, y, dir)
    } else if dir & DIR_LEFT != 0 {
        (x, y - 1, dir)
    } else {
        panic!("invalid direction");
    }
}

pub fn count_energy_tiles(map: &Map, entry: (i16, i16, Direction)) -> usize {
    let mut heads: Vec<(i16, i16, Direction)> = vec![entry];
    let mut visited: HashMap<(i16, i16), Direction> = HashMap::default();

    while let Some((x, y, dir)) = heads.pop() {
        // Abort if out-of-bounds
        if x < 0 || x >= map.rows() || y < 0 || y >= map.columns() {
            continue;
        }

        // Abort if already visited
        let visited_directions_here = visited.entry((x, y)).or_default();
        if *visited_directions_here & dir != 0 {
            continue;
        }
        *visited_directions_here |= dir;

        // Advance the beam head
        match map.0[x as usize][y as usize] {
            b'/' => {
                let new_dir = if dir & DIR_UP != 0 {
                    DIR_RIGHT
                } else if dir & DIR_LEFT != 0 {
                    DIR_DOWN
                } else if dir & DIR_RIGHT != 0 {
                    DIR_UP
                } else if dir & DIR_DOWN != 0 {
                    DIR_LEFT
                } else {
                    panic!("invalid direction")
                };
                heads.push(apply_dir(x, y, new_dir));
            }

            b'\\' => {
                let new_dir = if dir & DIR_UP != 0 {
                    DIR_LEFT
                } else if dir & DIR_RIGHT != 0 {
                    DIR_DOWN
                } else if dir & DIR_LEFT != 0 {
                    DIR_UP
                } else if dir & DIR_DOWN != 0 {
                    DIR_RIGHT
                } else {
                    panic!("invalid direction")
                };
                heads.push(apply_dir(x, y, new_dir));
            }

            b'|' => {
                if dir & (DIR_LEFT | DIR_RIGHT) != 0 {
                    heads.push(apply_dir(x, y, DIR_UP));
                    heads.push(apply_dir(x, y, DIR_DOWN));
                } else {
                    heads.push(apply_dir(x, y, dir));
                }
            }

            b'-' => {
                if dir & (DIR_UP | DIR_DOWN) != 0 {
                    heads.push(apply_dir(x, y, DIR_LEFT));
                    heads.push(apply_dir(x, y, DIR_RIGHT));
                } else {
                    heads.push(apply_dir(x, y, dir));
                }
            }

            _ => {
                heads.push(apply_dir(x, y, dir));
            }
        }
    }

    return visited.len();
}

pub fn load_input() -> Map {
    Map(stdin()
        .lines()
        .map(|l| l.expect("failed to read from stdin").into_bytes())
        .collect())
}
