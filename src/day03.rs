// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::io::stdin;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point(pub i16, pub i16);

impl Point {
    pub fn neighbors(self) -> [Self; 8] {
        [
            Point(self.0 - 1, self.1 - 1),
            Point(self.0 - 1, self.1),
            Point(self.0 - 1, self.1 + 1),
            Point(self.0, self.1 - 1),
            Point(self.0, self.1 + 1),
            Point(self.0 + 1, self.1 - 1),
            Point(self.0 + 1, self.1),
            Point(self.0 + 1, self.1 + 1),
        ]
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Number {
    pub value: u32,
    pub pt: Point,
    pub digits: u8,
}

impl Number {
    pub fn adjacent<'a>(&'a self) -> impl Iterator<Item = Point> + 'a {
        let col_start = self.pt.1 - 1;
        let col_end = self.pt.1 + self.digits as i16;

        (col_start..=col_end)
            .flat_map(|col| [Point(self.pt.0 - 1, col), Point(self.pt.0 + 1, col)])
            .chain([Point(self.pt.0, col_start), Point(self.pt.0, col_end)])
    }
}

pub fn load_input() -> (Vec<Number>, HashMap<Point, u8>) {
    let mut numbers = Vec::default();
    let mut symbols = HashMap::default();

    for (row, line) in stdin().lines().enumerate() {
        let line = line.expect("failed to read from stdin");
        let mut num_start: Option<usize> = None;

        for (col, &byte) in line.as_bytes().iter().enumerate() {
            if byte.is_ascii_digit() {
                if let None = num_start {
                    num_start = Some(col);
                }
            } else {
                if let Some(start) = num_start {
                    num_start = None;
                    let value = u32::from_str_radix(&line[start..col], 10).unwrap();
                    numbers.push(Number {
                        value,
                        pt: Point(row as i16, start as i16),
                        digits: (col - start) as u8,
                    });
                }

                if byte != b'.' {
                    symbols.insert(Point(row as i16, col as i16), byte);
                }
            }
        }

        // Special case for numbers at the very end of the line
        if let Some(num_start) = num_start {
            let value = u32::from_str_radix(&line[num_start..], 10).unwrap();
            numbers.push(Number {
                value,
                pt: Point(row as i16, num_start as i16),
                digits: (line.len() - num_start) as u8,
            })
        }
    }

    (numbers, symbols)
}
