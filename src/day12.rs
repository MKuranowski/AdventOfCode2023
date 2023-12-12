// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use std::io::stdin;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Condition {
    Operational = 0,
    Damaged = 1,
    Unknown = 2,
}

impl Condition {
    pub fn from_input_byte(value: u8) -> Self {
        match value {
            b'.' => Self::Operational,
            b'#' => Self::Damaged,
            b'?' => Self::Unknown,
            _ => panic!("unknown condition byte {}", value as char),
        }
    }

    pub fn from_mask(value: u32) -> Self {
        if value == 0 {
            Self::Operational
        } else {
            Self::Damaged
        }
    }
}

#[derive(Debug)]
pub struct SpringsRow {
    pub conditions: Vec<Condition>,
    pub broken_groups: Vec<u8>,
}

pub fn load_input() -> Vec<SpringsRow> {
    stdin()
        .lines()
        .map(|line| {
            let line = line.expect("failed to read from stdin");
            let (condition_str, broken_groups_str) = line.split_once(' ').unwrap();

            let conditions = condition_str
                .as_bytes()
                .iter()
                .copied()
                .map(Condition::from_input_byte)
                .collect();

            let broken_groups = broken_groups_str
                .split(',')
                .map(|s| u8::from_str_radix(s, 10).unwrap())
                .collect();

            SpringsRow {
                conditions,
                broken_groups,
            }
        })
        .collect()
}
