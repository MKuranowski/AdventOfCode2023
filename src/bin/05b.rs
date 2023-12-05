// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use aoc2023::day05::{load_input, CopyableRange};

fn seed_ranges(num: &[i64]) -> Vec<CopyableRange> {
    let starts = num.iter().step_by(2);
    let lengths = num.iter().skip(1).step_by(2);
    starts
        .zip(lengths)
        .map(|(&start, &len)| CopyableRange {
            start,
            end: start + len,
        })
        .collect()
}

fn main() {
    let (seed_ranges_flattened, almanac) = load_input();
    let seed_ranges = seed_ranges(&seed_ranges_flattened);

    let result = seed_ranges
        .iter()
        .flat_map(|&i| almanac.get_range(i))
        .map(|i| i.start)
        .min()
        .unwrap();

    println!("{result}");
}
