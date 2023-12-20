// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use aoc2023::day20::{load_input, Collector, ModuleID};

struct RxCollector {
    round: usize,
    low_to_rx: bool,
    rx_id: ModuleID,
}

impl RxCollector {
    fn new(rx_id: ModuleID) -> Self {
        Self {
            round: 0,
            low_to_rx: false,
            rx_id,
        }
    }
}

impl Collector for RxCollector {
    fn on_click(&mut self) {
        self.round += 1;
    }

    fn on_pulse(&mut self, _from: ModuleID, to: ModuleID, is_high: bool) {
        if to == self.rx_id && !is_high {
            self.low_to_rx = true;
        }
    }
}

fn main() {
    let mut system = load_input();
    let mut collector = RxCollector::new(*system.name_to_id.get("rx").unwrap());

    while !collector.low_to_rx {
        if collector.round % 100_000 == 0 {
            eprintln!("{}", collector.round);
        }

        system.click_button(&mut collector);
    }

    println!("{}", collector.round);
}
