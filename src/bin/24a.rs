// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use std::io::stdin;

#[derive(Debug, Clone, Copy)]
struct Hailstone {
    px: f64,
    py: f64,
    vx: f64,
    vy: f64,

    line_a: f64,
    line_b: f64,
    line_c: f64,
}

impl Hailstone {
    fn new(px: f64, py: f64, vx: f64, vy: f64) -> Self {
        let line_a = -vy;
        let line_b = vx;
        let line_c = px * vy - py * vx;
        Self {
            px,
            py,
            vx,
            vy,
            line_a,
            line_b,
            line_c,
        }
    }

    fn intersection_with(&self, other: &Self) -> Option<(f64, f64)> {
        // https://www.math.edu.pl/punkt-przeciecia-dwoch-prostych

        // let denominator = self.vx * other.vy - self.vy * other.vx;
        let denominator = self.line_a * other.line_b - other.line_a * self.line_b;
        if is_close(denominator, 0.0) {
            return None;
        }

        // let a = (self.px + self.vx) * self.py - (self.py + self.vy) * self.px;
        // let b = (other.px + other.vx) * other.py - (other.py + other.vy) * other.px;
        // let x_numerator = a * other.vx - self.vx * b;
        // let y_numerator = a * other.vy - self.vy * b;
        let x_numerator = -self.line_c * other.line_b - -other.line_c * self.line_b;
        let y_numerator = self.line_a * -other.line_c - other.line_a * -self.line_c;

        Some((x_numerator / denominator, y_numerator / denominator))
    }

    fn in_future(&self, (px2, py2): (f64, f64)) -> bool {
        in_future(self.px, px2, self.vx) && in_future(self.py, py2, self.vy)
    }
}

fn in_future(x1: f64, x2: f64, dx: f64) -> bool {
    x1 == x2 || (dx > 0.0 && x2 > x1) || (dx < 0.0 && x2 < x1)
}

fn is_close(x: f64, to: f64) -> bool {
    const MAX_DELTA: f64 = 1e-9;
    (to - x).abs() < MAX_DELTA
}

fn load_input() -> Vec<Hailstone> {
    stdin()
        .lines()
        .map(|l| {
            let l = l.expect("failed to read from stdin");
            let (pos_str, velocity_str) = l.split_once(" @ ").unwrap();

            let mut positions = pos_str.split(", ");
            let px = i64::from_str_radix(positions.next().unwrap(), 10).unwrap() as f64;
            let py = i64::from_str_radix(positions.next().unwrap(), 10).unwrap() as f64;

            let mut velocities = velocity_str.split(", ");
            let vx = i64::from_str_radix(velocities.next().unwrap(), 10).unwrap() as f64;
            let vy = i64::from_str_radix(velocities.next().unwrap(), 10).unwrap() as f64;

            Hailstone::new(px, py, vx, vy)
        })
        .collect()
}

fn main() {
    let hailstones = load_input();
    let (min_coord, max_coord) = if hailstones.len() == 5 {
        (7.0, 27.0)
    } else {
        (200000000000000.0, 400000000000000.0)
    };

    let mut result: usize = 0;

    for i in 0..hailstones.len() {
        let a = &hailstones[i];

        for j in (i + 1)..hailstones.len() {
            let b = &hailstones[j];

            if let Some(intersection) = a.intersection_with(b) {
                if a.in_future(intersection)
                    && b.in_future(intersection)
                    && intersection.0 >= min_coord
                    && intersection.1 >= min_coord
                    && intersection.0 <= max_coord
                    && intersection.1 <= max_coord
                {
                    result += 1;
                }
            }
        }
    }

    // XXX: 25787 is wrong

    println!("{result}");
}
