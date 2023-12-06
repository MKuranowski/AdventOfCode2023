// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use std::ops::Range;

#[derive(Debug, Default, Clone, Copy)]
pub struct Race {
    pub time: usize,
    pub distance: usize,
}

impl Race {
    pub fn winning_range(self) -> Range<usize> {
        // Formula for how far we go in a given race:
        // f(x) = (T - x)x = -x² + Tx
        //    x - time spent holding the button
        //    T - allowed time
        //
        // To figure out how long the button needs to be held the following must be solved:
        // f(x) > D <=> -x² + Tx > D <=> -x² + Tx - D > 0
        //    D - current record distance
        //
        // This is a simple quadratic equation.

        // Δ = b² - 4ac = T² - 4*(-1)*(-D) = T² - 4D
        // Assume we can always win the race, thus Δ >= 0.
        let time_squared = (self.time as f64) * (self.time as f64);
        let delta = time_squared - 4.0 * self.distance as f64;

        // x_1 = (-b - √Δ) / 2a = (-T - √Δ) / -2 = (T + √Δ) / 2
        // x_2 = (-b + √Δ) / 2a = (-T + √Δ) / -2 = (T - √Δ) / 2
        let delta_sqrt = (delta as f64).sqrt();
        let left_f = (self.time as f64 - delta_sqrt) / 2.0;
        let right_f = (self.time as f64 + delta_sqrt) / 2.0;

        let left = left_f.floor() as usize + 1;
        let right = right_f.ceil() as usize;

        left..right
    }
}
