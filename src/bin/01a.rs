// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use std::io::*;

fn main() {
    let result: u32 = stdin()
        .lines()
        .map(|line| {
            let line = line.expect("failed to read from stdin");
            let first_idx = line
                .find(|c: char| c.is_ascii_digit())
                .expect("no digit in line");
            let first = line.as_bytes()[first_idx] as u8 - b'0';

            let last_idx = line
                .rfind(|c: char| c.is_ascii_digit())
                .expect("no digit in line");
            let last = line.as_bytes()[last_idx] as u8 - b'0';

            (first * 10 + last) as u32
        })
        .sum();

    println!("{result}");
}
