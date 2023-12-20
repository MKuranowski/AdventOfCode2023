// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use aoc2023::day20::{load_input, Collector, ModuleID};

#[derive(Default)]
struct KindCounter {
    low: usize,
    high: usize,
}

impl Collector for KindCounter {
    fn on_pulse(&mut self, _from: ModuleID, _to: ModuleID, is_high: bool) {
        if is_high {
            self.high += 1;
        } else {
            self.low += 1;
        }
    }
}

fn main() {
    let mut system = load_input();
    let mut counter = KindCounter::default();

    for _ in 0..1000 {
        system.click_button(&mut counter);
    }

    let result = counter.low * counter.high;
    println!("{result}");
}
