// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use std::io::stdin;

pub fn extrapolate(initial: Vec<i32>) -> i32 {
    let mut stack = vec![initial];
    while !stack.last().unwrap().iter().all(|&i| i == 0) {
        stack.push(
            stack
                .last()
                .unwrap()
                .windows(2)
                .map(|w| w[1] - w[0])
                .collect(),
        );
    }

    for i in (0..stack.len()).rev() {
        if i == stack.len() - 1 {
            stack[i].push(0);
        } else {
            let last = *stack[i].last().unwrap();
            let delta = *stack[i + 1].last().unwrap();
            stack[i].push(last + delta);
        }
    }

    *stack.first().unwrap().last().unwrap()
}

pub fn load_input() -> impl Iterator<Item = Vec<i32>> {
    stdin().lines().map(|l| {
        l.expect("failed to read from stdin")
            .split_ascii_whitespace()
            .map(|d| i32::from_str_radix(d, 10).unwrap())
            .collect::<Vec<i32>>()
    })
}
