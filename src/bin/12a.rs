// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use aoc2023::day12::{load_input, Condition, SpringsRow};

struct SubstitutionGenerator<'a> {
    original: &'a [Condition],
    current: u32,
    max: u32,
}

impl<'a> SubstitutionGenerator<'a> {
    fn new(original: &'a [Condition]) -> Self {
        let unknown_count = original
            .iter()
            .filter(|&c| *c == Condition::Unknown)
            .count() as u32;

        Self {
            original,
            current: 0,
            max: 1 << unknown_count,
        }
    }

    fn next(&mut self, target: &mut [Condition]) -> bool {
        if self.current >= self.max {
            return false;
        }

        assert!(self.original.len() == target.len());

        let mut mask = self.current;
        for (i, &original_condition) in self.original.iter().enumerate() {
            if original_condition == Condition::Unknown {
                target[i] = Condition::from_mask(mask & 1);
                mask >>= 1;
            } else {
                debug_assert!(original_condition == target[i]);
            }
        }

        self.current += 1;
        return true;
    }
}

fn is_consistent(conditions: &[Condition], expected_broken_groups: &[u8]) -> bool {
    let got_broken_groups = conditions
        .split(|&c| c == Condition::Operational)
        .filter(|&group| !group.is_empty())
        .map(|group| group.len() as u8);

    got_broken_groups.eq(expected_broken_groups.iter().copied())
}

fn count_consistent_arrangements(row: &SpringsRow) -> usize {
    let mut substitutions = SubstitutionGenerator::new(&row.conditions);
    let mut substituted = row.conditions.clone();
    let mut count = 0;

    while substitutions.next(&mut substituted) {
        if is_consistent(&substituted, &row.broken_groups) {
            count += 1;
        }
    }

    return count;
}

fn main() {
    let result = load_input()
        .iter()
        .map(count_consistent_arrangements)
        .sum::<usize>();
    println!("{result}");
}
