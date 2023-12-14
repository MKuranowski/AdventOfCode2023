// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use std::collections::HashMap;

use aoc2023::day14::{load_input, Platform};

fn find_platform_by_cycle<'a>(
    cycle_by_platform: &'a HashMap<Platform, usize>,
    target_cycle: usize,
) -> &'a Platform {
    for (platform, cycle) in cycle_by_platform {
        if *cycle == target_cycle {
            return platform;
        }
    }
    panic!("Cycle not found: {target_cycle}");
}

fn main() {
    const TARGET_CYCLE: usize = 1000000000;
    let mut platform = load_input();
    let mut cycle_by_platform: HashMap<Platform, usize> = HashMap::default();

    for current_cycle in 0..1_000_000 {
        if let Some(previous_cycle) = cycle_by_platform.get(&platform) {
            // Platforms repeat - find the state equivalent to target
            let period = current_cycle - previous_cycle;
            let equivalent_cycle = previous_cycle + ((TARGET_CYCLE - previous_cycle) % period);
            let target_platform = find_platform_by_cycle(&cycle_by_platform, equivalent_cycle);
            let result = target_platform.north_load();
            println!("{result}");
            return;
        } else {
            cycle_by_platform.insert(platform.clone(), current_cycle);
        }

        platform.cycle();
    }

    panic!("Platforms don't repeat after a reasonable amount of cycles :^(");
}
