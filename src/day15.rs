// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use std::io::{stdin, Read};
use std::ops::Deref;

pub fn run_hash<I: IntoIterator<Item = T>, T: Deref<Target = u8>>(i: I) -> u8 {
    i.into_iter()
        .fold(0, |value, elem| value.wrapping_add(*elem).wrapping_mul(17))
}

pub fn load_input() -> String {
    let mut content = String::default();
    stdin()
        .read_to_string(&mut content)
        .expect("failed to read from stdin");

    // WHAT THE FUCK RUST, WHY THERE ARE NO EQUIVALENT IN-PLACE FUNCTIONS?????
    content.trim().replace("\n", ",")
}
