// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use crate::bitset::SmallBitset;
use std::io::stdin;

fn parse_numbers(numbers: &str) -> SmallBitset {
    let mut s = SmallBitset::default();
    for n in numbers.split_ascii_whitespace() {
        let n = u32::from_str_radix(n, 10).unwrap();
        s.insert(n);
    }
    s
}

fn parse_line(line: &str) -> (u32, SmallBitset, SmallBitset) {
    let (prefix, numbers_str) = line.split_once(": ").unwrap();
    let card_id_str = prefix.split_once(' ').unwrap().1.trim_start();
    let card_id = u32::from_str_radix(card_id_str, 10).unwrap();

    let (expected_str, got_str) = numbers_str.split_once(" | ").unwrap();
    let expected = parse_numbers(expected_str);
    let got = parse_numbers(got_str);

    (card_id, expected, got)
}

pub fn load_input() -> impl Iterator<Item = (u32, SmallBitset, SmallBitset)> {
    stdin()
        .lines()
        .map(|line| parse_line(&line.expect("failed to read from stdin")))
}

pub fn calc_score(expected: SmallBitset, got: SmallBitset) -> u32 {
    let overlap = expected.intersection(got).len();
    if overlap == 0 {
        0
    } else {
        1 << (overlap as u32 - 1)
    }
}
