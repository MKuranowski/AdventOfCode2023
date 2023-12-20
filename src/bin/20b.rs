// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use aoc2023::day20::{load_input, Collector, Module, ModuleID, ModuleKind, System};
use num::integer::lcm;

struct ActivationCollector {
    round: usize,
    activated: bool,
    target_id: ModuleID,
}

impl ActivationCollector {
    fn new(target_id: ModuleID) -> Self {
        Self {
            round: 0,
            activated: false,
            target_id,
        }
    }
}

impl Collector for ActivationCollector {
    fn on_click(&mut self) {
        self.round += 1;
    }

    fn on_pulse(&mut self, _from: ModuleID, to: ModuleID, is_high: bool) {
        if to == self.target_id && !is_high {
            self.activated = true;
        }
    }
}

fn rounds_till_activated(system: &mut System, target_id: ModuleID) -> usize {
    system.reset();
    let mut c = ActivationCollector::new(target_id);
    while !c.activated {
        system.click_button(&mut c);
    }
    return c.round;
}

fn first_parent(system: &System, id: ModuleID) -> &Module {
    for module in &system.modules {
        if module.children.contains(&id) {
            return module;
        }
    }
    panic!("no parent of {}", id);
}

fn main() {
    // The graph of modules ends with the following schema:
    //
    // &xx &yy &zz &ww
    //   \   ↓ ↓   /
    //    -→ &aa ←-
    //        ↓
    //       rx
    //
    // Brute-forcing the amount of button presses required to send low to "rx"
    // is unfeasible. However, thanks to the above structure, it can be
    // calculated as the LCM of button presses required to send low to "xx", "yy", "zz" and "ww".
    // Those amounts in turn can be easily brute-forced.

    let mut system = load_input();

    let children_ids: Vec<u16> = {
        let rx_parent = first_parent(&system, *system.name_to_id.get("rx").unwrap());
        match rx_parent.kind {
            ModuleKind::Conjunction(ref inputs) => inputs.keys().copied().collect(),
            _ => panic!("panic of \"rx\" is not a conjunction!"),
        }
    };

    let result = children_ids
        .iter()
        .map(|&child_id| rounds_till_activated(&mut system, child_id))
        .fold(1_usize, |a, b| lcm(a, b));

    println!("{result}");
}
