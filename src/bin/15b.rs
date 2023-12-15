// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use std::collections::HashMap;

use aoc2023::day15::{load_input, run_hash};

#[derive(Debug)]
struct Program {
    boxes: Vec<Vec<String>>,
    lens_to_focal_length: HashMap<String, u8>,
}

impl Default for Program {
    fn default() -> Self {
        Self {
            boxes: vec![Vec::default(); 256],
            lens_to_focal_length: HashMap::default(),
        }
    }
}

impl Program {
    fn exec_dash(&mut self, label: &str) {
        let box_idx = run_hash(label.as_bytes());
        if let Some(lens_idx) = self.boxes[box_idx as usize]
            .iter()
            .position(|i| i == &label)
        {
            self.boxes[box_idx as usize].remove(lens_idx);
        }
    }

    fn exec_equals(&mut self, label: &str, focal_length: u8) {
        let box_idx = run_hash(label.as_bytes());
        if self.boxes[box_idx as usize]
            .iter()
            .position(|i| i == label)
            .is_none()
        {
            self.boxes[box_idx as usize].push(label.to_string());
        }

        if let Some(stored_focal_length) = self.lens_to_focal_length.get_mut(label) {
            *stored_focal_length = focal_length;
        } else {
            self.lens_to_focal_length
                .insert(label.to_string(), focal_length);
        }
    }

    fn exec(&mut self, command: &str) {
        if command.ends_with('-') {
            self.exec_dash(&command[0..command.len() - 1]);
        } else {
            let (label, focal_length_str) = command.split_once('=').unwrap();
            let focal_length = u8::from_str_radix(focal_length_str, 10).unwrap();
            self.exec_equals(label, focal_length);
        }
    }

    fn total_focusing_power(&self) -> usize {
        self.boxes
            .iter()
            .enumerate()
            .flat_map(|(box_idx, box_)| {
                box_.iter().enumerate().map(move |(lens_idx, lens)| {
                    let focal_length = *self.lens_to_focal_length.get(lens).unwrap() as usize;
                    focal_length * (box_idx + 1) * (lens_idx + 1)
                })
            })
            .sum()
    }
}

fn main() {
    let mut program = Program::default();
    for instruction in load_input().split(',') {
        program.exec(instruction);
    }
    let result = program.total_focusing_power();
    println!("{result}");
}
