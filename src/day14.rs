// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use std::fmt::{Debug, Write};
use std::io::stdin;
use std::str::from_utf8_unchecked;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Platform(Vec<Vec<u8>>);

impl Debug for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
            // SAFETY: Who cares if the output is not valid utf-8, the computer is not gonna blow up
            f.write_str(unsafe { from_utf8_unchecked(row) })?;
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl Platform {
    pub fn north_load(&self) -> usize {
        let rows = self.0.len();
        self.0
            .iter()
            .enumerate()
            .map(|(row_idx, row)| {
                let row_score = rows - row_idx;
                row_score * row.iter().filter(|&&b| b == b'O').count()
            })
            .sum::<usize>()
    }

    fn column_tilt_north(&mut self, col: usize) {
        let mut insert_row: usize = 0;

        for row in 0..self.0.len() {
            match self.0[row][col] {
                b'O' => {
                    if row != insert_row {
                        self.0[row][col] = b'.';
                        self.0[insert_row][col] = b'O';
                    }
                    insert_row += 1;
                }
                b'#' => {
                    insert_row = row + 1;
                }
                _ => {}
            }
        }
    }

    pub fn tilt_north(&mut self) {
        for col in 0..self.0[0].len() {
            self.column_tilt_north(col);
        }
    }

    fn column_tilt_south(&mut self, col: usize) {
        let mut insert_row: usize = self.0.len() - 1;

        for row in (0..self.0.len()).rev() {
            match self.0[row][col] {
                b'O' => {
                    if row != insert_row {
                        self.0[row][col] = b'.';
                        self.0[insert_row][col] = b'O';
                    }
                    insert_row -= 1;
                }
                b'#' => {
                    insert_row = row - 1;
                }
                _ => {}
            }
        }
    }

    pub fn tilt_south(&mut self) {
        for col in 0..self.0[0].len() {
            self.column_tilt_south(col);
        }
    }

    fn row_tilt_west(&mut self, row: usize) {
        let mut insert_col: usize = 0;

        for col in 0..self.0[0].len() {
            match self.0[row][col] {
                b'O' => {
                    if col != insert_col {
                        self.0[row][col] = b'.';
                        self.0[row][insert_col] = b'O';
                    }
                    insert_col += 1;
                }
                b'#' => {
                    insert_col = col + 1;
                }
                _ => {}
            }
        }
    }

    pub fn tilt_west(&mut self) {
        for row in 0..self.0.len() {
            self.row_tilt_west(row);
        }
    }

    fn row_tilt_east(&mut self, row: usize) {
        let mut insert_col: usize = self.0[0].len() - 1;

        for col in (0..self.0[0].len()).rev() {
            match self.0[row][col] {
                b'O' => {
                    if col != insert_col {
                        self.0[row][col] = b'.';
                        self.0[row][insert_col] = b'O';
                    }
                    insert_col -= 1;
                }
                b'#' => {
                    insert_col = col - 1;
                }
                _ => {}
            }
        }
    }

    pub fn tilt_east(&mut self) {
        for row in 0..self.0.len() {
            self.row_tilt_east(row);
        }
    }

    pub fn cycle(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }
}

pub fn load_input() -> Platform {
    Platform(
        stdin()
            .lines()
            .map(|l| l.expect("failed to read from stdin").into_bytes())
            .collect(),
    )
}
