// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use std::collections::HashMap;

use aoc2023::day03::*;

fn numbers_by_point<'a>(numbers: &'a [Number]) -> HashMap<Point, &'a Number> {
    let mut by_point = HashMap::default();
    for number in numbers {
        for col_offset in 0..number.digits {
            by_point.insert(
                Point(number.pt.0, number.pt.1 + col_offset as i16),
                number);
        }
    }
    by_point
}

fn main() {
    let (numbers, points) = load_input();
    let by_point = numbers_by_point(&numbers);
    let mut result: u32 = 0;

    for (&point, &symbol) in points.iter() {
        if symbol != b'*' {
            continue;
        }

        let adjacent_numbers: HashMap<Point, Number> = point
            .neighbors()
            .iter()
            .filter_map(|neighbor| by_point.get(neighbor))
            .map(|&number| (number.pt, *number))
            .collect();

        if adjacent_numbers.len() == 2 {
            result += adjacent_numbers.values().map(|n| n.value).product::<u32>();
        }
    }

    println!("{result}");
}
