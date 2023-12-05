// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use std::io::stdin;

#[derive(Debug, Clone, Copy)]
pub struct CopyableRange {
    pub start: i64,
    pub end: i64,
}

impl Into<std::ops::Range<i64>> for CopyableRange {
    fn into(self) -> std::ops::Range<i64> {
        self.start..self.end
    }
}

impl CopyableRange {
    pub fn contains(self, i: i64) -> bool {
        i >= self.start && i < self.end
    }

    pub fn intersection(self, other: Self) -> Option<Self> {
        if self.start >= other.end {
            None
        } else if self.end <= other.start {
            None
        } else {
            Some(CopyableRange {
                start: self.start.max(other.start),
                end: self.end.min(other.end),
            })
        }
    }

    pub fn add(self, delta: i64) -> Self {
        Self {
            start: self.start + delta,
            end: self.end + delta,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MapEntry {
    pub src: CopyableRange,
    pub delta: i64,
}

impl MapEntry {
    pub fn get(&self, i: i64) -> Option<i64> {
        if self.src.contains(i) {
            Some(i + self.delta)
        } else {
            None
        }
    }
}

#[derive(Debug, Default)]
pub struct Map(Vec<MapEntry>);

impl Map {
    pub fn get(&self, i: i64) -> i64 {
        for entry in &self.0 {
            if let Some(j) = entry.get(i) {
                return j;
            }
        }
        return i;
    }

    pub fn get_range<'a>(&'a self, i: CopyableRange) -> impl Iterator<Item = CopyableRange> + 'a {
        MapPartition::new(i, self.0.iter())
    }
}

struct MapPartition<'a, I: Iterator<Item=&'a MapEntry>> {
    next_entries: I,
    entry: Option<&'a MapEntry>,
    start: i64,
    end: i64,
}

impl<'a, I: Iterator<Item=&'a MapEntry>> MapPartition<'a, I> {
    fn new(r: CopyableRange, mut entries: I) -> Self {
        let first = entries.next();
        Self {
            next_entries: entries,
            entry: first,
            start: r.start,
            end: r.end,
        }
    }
}

impl<'a, I: Iterator<Item=&'a MapEntry>> Iterator for MapPartition<'a, I> {
    type Item = CopyableRange;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start >= self.end {
            return None;
        }

        if self.entry.is_none() {
            let r = CopyableRange { start: self.start, end: self.end };
            self.start = self.end;
            return Some(r);
        }

        let entry = self.entry.unwrap();
        if self.start < entry.src.start {
            let r = CopyableRange { start: self.start, end: self.end.min(entry.src.start) };
            self.start = entry.src.start;
            return Some(r);
        }

        if self.start < entry.src.end {
            let r = CopyableRange { start: self.start, end: self.end.min(entry.src.end) }.add(entry.delta);
            self.start = entry.src.end;
            self.entry = self.next_entries.next();
            return Some(r);
        }

        self.entry = self.next_entries.next();
        return self.next();  // why bother with loops when you have recursion
    }
}


#[derive(Debug, Default)]
pub struct Almanac(Vec<Map>);

impl Almanac {
    pub fn get(&self, i: i64) -> i64 {
        self.0.iter().fold(i, |i, map| map.get(i))
    }

    pub fn get_range(&self, i: CopyableRange) -> Vec<CopyableRange> {
        let mut ranges = vec![i];
        for map in &self.0 {
            ranges = ranges.iter().flat_map(|&i| map.get_range(i)).collect();
        }
        ranges
    }
}

pub fn load_input() -> (Vec<i64>, Almanac) {
    let mut seeds = Vec::default();
    let mut almanac = Almanac::default();

    for line in stdin().lines() {
        let line = line.expect("failed to read from stdin");

        if line.is_empty() {
            // Ignore empty lines
        } else if line.starts_with("seeds: ") {
            seeds = line
                .split_once(':')
                .unwrap()
                .1
                .split_ascii_whitespace()
                .map(|i| i64::from_str_radix(i, 10).unwrap())
                .collect();
        } else if line.ends_with(" map:") {
            almanac.0.push(Map::default());
        } else {
            let mut parts = line.split_ascii_whitespace();

            let dst_start = i64::from_str_radix(parts.next().unwrap(), 10).unwrap();
            let src_start = i64::from_str_radix(parts.next().unwrap(), 10).unwrap();
            let len = i64::from_str_radix(parts.next().unwrap(), 10).unwrap();

            almanac.0.last_mut().unwrap().0.push(MapEntry {
                src: CopyableRange {
                    start: src_start,
                    end: src_start + len,
                },
                delta: dst_start - src_start,
            });
        }
    }

    for map in almanac.0.iter_mut() {
        map.0.sort_by_key(|i| i.src.start);
    }

    (seeds, almanac)
}
