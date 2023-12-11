// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use std::io::stdin;

pub type Coords = [i32; 2];

pub fn load_input() -> Vec<Coords> {
    let mut galaxies = Vec::default();

    for (row, line) in stdin().lines().enumerate() {
        let line = line.expect("failed to read from stdin");
        for (col, &c) in line.as_bytes().iter().enumerate() {
            if c == b'#' {
                galaxies.push([row as i32, col as i32]);
            }
        }
    }

    galaxies
}

fn expand_axis(coords: &mut [Coords], axis: usize, factor: i32) {
    let mut last = coords.iter().map(|c| c[axis]).max().unwrap();
    let mut curr: i32 = 0;

    while curr < last {
        if coords.iter().any(|c| c[axis] == curr) {
            // There's a galaxy on `curr` axis - move to the next one
            curr += 1;
        } else {
            // No galaxies on `curr` axis - expand it
            coords
                .iter_mut()
                .filter(|c| c[axis] > curr)
                .for_each(|c| c[axis] += factor);
            curr += factor + 1;
            last += factor;
        }
    }
}

pub fn expand(coords: &mut [Coords], factor: i32) {
    expand_axis(coords, 0, factor);
    expand_axis(coords, 1, factor);
}

pub fn sum_distances(galaxies: &[Coords]) -> usize {
    let mut sum: usize = 0;
    for i in 0..galaxies.len() {
        let a = galaxies[i];
        for j in (i + 1)..galaxies.len() {
            let b = galaxies[j];
            let dist = (b[0] - a[0]).abs() + (b[1] - a[1]).abs();
            sum += dist as usize;
        }
    }
    sum
}
