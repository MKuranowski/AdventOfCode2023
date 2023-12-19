// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coords(pub i32, pub i32);

impl Coords {
    pub fn moved(self, dir: Direction, step: i32) -> Self {
        match dir {
            Direction::Up => Coords(self.0 - step, self.1),
            Direction::Right => Coords(self.0, self.1 + step),
            Direction::Down => Coords(self.0 + step, self.1),
            Direction::Left => Coords(self.0, self.1 - step),
        }
    }

    pub fn is_inside(self, polygon: &[Self]) -> bool {
        // https://en.wikipedia.org/wiki/Even%E2%80%93odd_rule
        let mut contained = false;
        for i in 0..polygon.len() {
            let j = if i > 0 { i - 1 } else { polygon.len() - 1 };

            // Check if self is a corner
            if self == polygon[i] {
                return true;
            }

            // Check if on x-aligned boundary
            if self.0 == polygon[i].0 && self.0 == polygon[j].0 {
                let left = polygon[i].1.min(polygon[j].1);
                let right = polygon[i].1.max(polygon[j].1);
                if self.1 >= left && self.1 <= right {
                    return true;
                }
            }

            // Check if on y-aligned boundary
            if self.1 == polygon[i].1 && self.1 == polygon[j].1 {
                let top = polygon[i].0.min(polygon[j].0);
                let bottom = polygon[i].0.max(polygon[j].0);
                if self.0 >= top && self.0 <= bottom {
                    return true;
                }
            }

            if (polygon[i].1 > self.1) != (polygon[j].1 > self.1) {
                let slope = (self.0 - polygon[i].0) as isize
                    * (polygon[j].1 - polygon[i].1) as isize
                    - (polygon[j].0 - polygon[i].0) as isize * (self.1 - polygon[i].1) as isize;

                // The slope == 0 check is replaced with the more robust checks above.

                if (slope < 0) != (polygon[j].1 < polygon[i].1) {
                    contained = !contained;
                }
            }
        }
        return contained;
    }
}

#[derive(Debug)]
pub struct PlanEntry {
    pub dir: Direction,
    pub step: i32,
}

#[derive(Debug, Default)]
pub struct Trench {
    pub corners: Vec<Coords>,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
    pub left: i32,
}

impl Trench {
    pub fn dig(&mut self, plan: &[PlanEntry]) {
        let start = Coords(0, 0);
        let mut pt = start;

        for i in plan {
            self.corners.push(pt);
            self.top = self.top.min(pt.0);
            self.bottom = self.bottom.max(pt.0);
            self.left = self.left.min(pt.1);
            self.right = self.right.max(pt.1);

            pt = pt.moved(i.dir, i.step);
        }

        assert!(pt == start);
        self.corners.reverse();
    }

    pub fn digged(plan: &[PlanEntry]) -> Self {
        let mut t = Self::default();
        t.dig(plan);
        t
    }
}
