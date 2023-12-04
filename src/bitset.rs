// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SmallBitset(u128);

impl SmallBitset {
    const MAX_VALUE: u32 = u128::BITS - 1;

    pub fn len(self) -> usize {
        self.0.count_ones() as usize
    }

    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub fn clear(&mut self) {
        self.0 = 0
    }

    pub fn contains(self, i: u32) -> bool {
        assert!(i < Self::MAX_VALUE);
        (self.0 >> i) & 1 != 0
    }

    pub fn insert(&mut self, i: u32) {
        assert!(i < Self::MAX_VALUE);
        self.0 |= 1 << i;
    }

    pub fn remove(&mut self, i: u32) {
        assert!(i < Self::MAX_VALUE);
        self.0 &= !(1 << i);
    }

    pub fn difference(self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }

    pub fn intersection(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    pub fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    pub fn is_disjoint(self, other: Self) -> bool {
        (self.0 & other.0) == 0
    }

    pub fn is_subset(self, other: Self) -> bool {
        (self.0 & other.0) == self.0
    }

    pub fn is_superset(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
}
