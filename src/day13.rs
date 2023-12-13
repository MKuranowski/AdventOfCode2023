// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use std::io::stdin;

pub type Image = Vec<Vec<u8>>;

pub fn is_symmetric_horizontal(img: &Image, cutoff: usize) -> bool {
    let to_examine = cutoff.min(img.len() - cutoff);
    for i in 0..to_examine {
        let before = &img[cutoff - i - 1];
        let after = &img[cutoff + i];
        if before != after {
            return false;
        }
    }
    return true;
}

fn are_columns_equal(img: &Image, a: usize, b: usize) -> bool {
    for row in img {
        if row[a] != row[b] {
            return false;
        }
    }
    return true;
}

pub fn is_symmetric_vertical(img: &Image, cutoff: usize) -> bool {
    let to_examine = cutoff.min(img[0].len() - cutoff);
    for i in 0..to_examine {
        if !are_columns_equal(img, cutoff - i - 1, cutoff + i) {
            return false;
        }
    }
    return true;
}

pub fn find_reflection_line(img: &Image) -> usize {
    for row in 1..img.len() {
        if is_symmetric_horizontal(img, row) {
            return 100 * row;
        }
    }

    for col in 1..img[0].len() {
        if is_symmetric_vertical(img, col) {
            return col;
        }
    }

    panic!("No symmetry in image!");
}

pub fn load_input() -> Vec<Image> {
    stdin()
        .lines()
        .map(|line| line.expect("failed to read from stdin").into_bytes())
        .collect::<Vec<_>>()
        .split(|line| line.is_empty())
        .map(|image| image.to_vec())
        .collect()
}
