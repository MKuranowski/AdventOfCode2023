// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

#[derive(Debug, Default)]
pub struct Bag {
    pub red: u32,
    pub blue: u32,
    pub green: u32
}

impl Bag {
    pub fn is_subset_of(&self, other: &Self) -> bool {
        self.red <= other.red && self.blue <= other.blue && self.green <= other.green
    }

    pub fn max(&self, other: &Self) -> Self {
        Self {
            red: self.red.max(other.red),
            blue: self.blue.max(other.blue),
            green: self.green.max(other.green),
        }
    }

    pub fn power(&self) -> u32 {
        self.red * self.blue * self.green
    }
}

fn parse_bag(bag_str: &str) -> Bag {
    let mut bag = Bag::default();
    for cube_str in bag_str.split(", ") {
        let (count_str, color) = cube_str.split_once(' ').unwrap();
        let count = u32::from_str_radix(count_str, 10).unwrap();
        match color {
            "red" => bag.red = count,
            "green" => bag.green = count,
            "blue" => bag.blue = count,
            _ => panic!("unrecognized color: {color}"),
        };
    }
    bag
}

fn parse_bags<'a>(bags_str: &'a str) -> impl Iterator<Item = Bag> + 'a {
    bags_str.split("; ").map(parse_bag)
}

pub fn parse_game<'a>(line: &'a str) -> (u32, impl Iterator<Item = Bag> + 'a) {
    let (game_id_str, bags_str) = line.split_once(": ").unwrap();
    let game_id = u32::from_str_radix(game_id_str.split_once(' ').unwrap().1, 10).unwrap();
    (game_id, parse_bags(bags_str))
}
