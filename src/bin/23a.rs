// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use std::collections::VecDeque;
use std::io::stdin;
use std::iter::once;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

struct Map(Vec<Vec<u8>>);

impl Map {
    fn rows(&self) -> usize {
        self.0.len()
    }

    fn columns(&self) -> usize {
        self.0[0].len()
    }

    fn fill_steps(&self) -> Vec<Vec<u16>> {
        let mut max_steps: Vec<Vec<u16>> = vec![vec![0; self.columns()]; self.rows()];
        let mut q: VecDeque<(u8, u8, u16, Direction)> = once((0, 1, 0, Direction::Up)).collect();

        while let Some((x, y, steps, came_from)) = q.pop_front() {
            let tile = self.0[x as usize][y as usize];

            // Don't walk over the forest
            if tile == b'#' {
                continue;
            }

            // Stop if a better path exists
            let best_steps = max_steps[x as usize][y as usize];
            if x > 0 && best_steps > steps {
                continue;
            }
            max_steps[x as usize][y as usize] = steps;

            // Go upwards
            if x > 0 && tile == b'.' && came_from != Direction::Up {
                q.push_back((x - 1, y, steps + 1, Direction::Down));
            }

            // Go right
            if y + 1 < self.columns() as u8
                && (tile == b'.' || tile == b'>')
                && came_from != Direction::Right
            {
                q.push_back((x, y + 1, steps + 1, Direction::Left));
            }

            // Go down
            if x + 1 < self.rows() as u8
                && (tile == b'.' || tile == b'v')
                && came_from != Direction::Down
            {
                q.push_back((x + 1, y, steps + 1, Direction::Up));
            }

            // Go left
            if y > 0 && tile == b'.' && came_from != Direction::Left {
                q.push_back((x, y - 1, steps + 1, Direction::Right));
            }
        }

        max_steps
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
    let steps = map.fill_steps();
    let result = steps[map.rows() - 1][map.columns() - 2];
    println!("{result}")
}
