// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::iter::once;

use aoc2023::day12::{load_input, Condition, SpringsRow};

#[derive(Default)]
struct Counter {
    cache: HashMap<Vec<Condition>, HashMap<Vec<u8>, usize>>,
}

impl Counter {
    fn cached_count(&self, conditions: &[Condition], groups: &[u8]) -> Option<usize> {
        self.cache
            .get(conditions)
            .map(|inner| inner.get(groups))
            .flatten()
            .copied()
    }

    fn put_to_cache(&mut self, conditions: &[Condition], groups: &[u8], count: usize) {
        self.cache
            .entry(conditions.to_vec())
            .or_default()
            .insert(groups.to_vec(), count);
    }

    fn count_possible_arrangements(&mut self, conditions: &[Condition], groups: &[u8]) -> usize {
        // Base recursion cases
        if conditions.len() == 0 {
            return if groups.len() == 0 { 1 } else { 0 };
        }
        if groups.len() == 0 {
            return if !conditions.contains(&Condition::Damaged) {
                1
            } else {
                0
            };
        }

        // Check in cache
        if let Some(cached_count) = self.cached_count(conditions, groups) {
            return cached_count;
        }

        // Compute the possible arrangements
        let mut result = 0;

        // On '.' (or '?' substituted by '.') - simply recurse skipping this spring,
        // as it doesn't contribute to any group
        if conditions[0] == Condition::Operational || conditions[0] == Condition::Unknown {
            result += self.count_possible_arrangements(&conditions[1..], groups);
        }

        // On '#' (or '?' substituted by '#') - check if the first group can be fulfilled and
        // if so, recurse past this group.
        if conditions[0] == Condition::Damaged || conditions[0] == Condition::Unknown {
            let group = groups[0] as usize;
            // A group can be fulfilled if:
            // 1. there are enough springs, and
            // 2. there are no operational springs in the group, and
            // 3. these are the last springs or the group is followed by an undamaged(/unknown) spring.
            if conditions.len() >= group
                && !(&conditions[..group]).contains(&Condition::Operational)
                && (conditions.len() == group || conditions[group] != Condition::Damaged)
            {
                result += self.count_possible_arrangements(
                    &conditions[conditions.len().min(group + 1)..],
                    &groups[1..],
                );
            }
        }

        self.put_to_cache(conditions, groups, result);
        return result;
    }
}

fn extend_row(row: &mut SpringsRow) {
    row.conditions = row
        .conditions
        .iter()
        .chain(once(&Condition::Unknown))
        .chain(row.conditions.iter())
        .chain(once(&Condition::Unknown))
        .chain(row.conditions.iter())
        .chain(once(&Condition::Unknown))
        .chain(row.conditions.iter())
        .chain(once(&Condition::Unknown))
        .chain(row.conditions.iter())
        .copied()
        .collect();
    row.broken_groups = row.broken_groups.repeat(5);
}

fn main() {
    let mut counter = Counter::default();
    let result = load_input()
        .iter_mut()
        .map(|row| {
            extend_row(row);
            counter.count_possible_arrangements(&row.conditions, &row.broken_groups)
        })
        .sum::<usize>();
    println!("{result}");
}
