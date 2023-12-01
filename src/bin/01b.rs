// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use std::io::*;

fn starts_with_digit(slice: &[u8]) -> Option<u8> {
    if slice.len() == 0 {
        None
    } else if slice[0].is_ascii_digit() {
        Some(slice[0] as u8 - b'0')
    } else if slice.starts_with(b"one") {
        Some(1)
    } else if slice.starts_with(b"two") {
        Some(2)
    } else if slice.starts_with(b"three") {
        Some(3)
    } else if slice.starts_with(b"four") {
        Some(4)
    } else if slice.starts_with(b"five") {
        Some(5)
    } else if slice.starts_with(b"six") {
        Some(6)
    } else if slice.starts_with(b"seven") {
        Some(7)
    } else if slice.starts_with(b"eight") {
        Some(8)
    } else if slice.starts_with(b"nine") {
        Some(9)
    } else {
        None
    }
}

fn find_first_digit(line: &[u8]) -> u8 {
    for i in 0..line.len() {
        if let Some(digit) = starts_with_digit(&line[i..line.len()]) {
            return digit;
        }
    }
    panic!("no digit");
}

fn find_last_digit(line: &[u8]) -> u8 {
    for i in (0..line.len()).rev() {
        if let Some(digit) = starts_with_digit(&line[i..line.len()]) {
            return digit;
        }
    }
    panic!("no digit");
}

fn main() {
    let result: u32 = stdin()
        .lines()
        .map(|line| {
            let line = line.expect("failed to read from stdin");
            let first = find_first_digit(line.as_bytes());
            let last = find_last_digit(line.as_bytes());

            (first * 10 + last) as u32
        })
        .sum();

    println!("{result}");
}
