// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use aoc2023::day13::{
    find_reflection_line, is_symmetric_horizontal, is_symmetric_vertical, load_input, Image,
};

fn swap_char(x: &mut u8) {
    *x = match *x {
        b'.' => b'#',
        b'#' => b'.',
        y => y,
    }
}

fn find_reflection_line_thats_different(img: &Image, original: usize) -> Option<usize> {
    for row in 1..img.len() {
        if is_symmetric_horizontal(img, row) {
            let current = 100 * row;
            if current != original {
                return Some(current);
            }
        }
    }

    for col in 1..img[0].len() {
        if is_symmetric_vertical(img, col) {
            if col != original {
                return Some(col);
            }
        }
    }

    return None;
}

fn find_reflection_line_without_smudge(img: &mut Image) -> usize {
    let original = find_reflection_line(img);

    for i in 0..img.len() {
        for j in 0..img[0].len() {
            swap_char(&mut img[i][j]);

            if let Some(current) = find_reflection_line_thats_different(img, original) {
                return current;
            }

            swap_char(&mut img[i][j]);
        }
    }

    panic!("No alternative symmetry!")
}

fn main() {
    let result = load_input()
        .iter_mut()
        .map(find_reflection_line_without_smudge)
        .sum::<usize>();
    println!("{result}");
}
