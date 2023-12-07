// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use std::array;
use std::collections::HashMap;
use std::io::stdin;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

impl From<u8> for Card {
    fn from(value: u8) -> Self {
        match value {
            b'2' => Self::Two,
            b'3' => Self::Three,
            b'4' => Self::Four,
            b'5' => Self::Five,
            b'6' => Self::Six,
            b'7' => Self::Seven,
            b'8' => Self::Eight,
            b'9' => Self::Nine,
            b'T' => Self::T,
            b'J' => Self::J,
            b'Q' => Self::Q,
            b'K' => Self::K,
            b'A' => Self::A,
            _ => panic!("invalid card byte: {value:x}"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Hand(pub HandType, pub [Card; 5]);

impl Hand {
    pub fn new(cards: [Card; 5]) -> Self {
        let typ = Self::type_of(&cards);
        Self(typ, cards)
    }

    fn type_of(cards: &[Card; 5]) -> HandType {
        let card_counts = Self::count_cards(cards);
        match card_counts.len() {
            1 => HandType::FiveOfAKind,
            2 => {
                if card_counts.values().any(|&count| count == 1 || count == 4) {
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            3 => {
                if card_counts.values().any(|&count| count == 3) {
                    HandType::ThreeOfAKind
                } else {
                    HandType::TwoPair
                }
            }
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            _ => panic!("can't have more than 5 cards"),
        }
    }

    fn count_cards(cards: &[Card; 5]) -> HashMap<Card, u8> {
        let mut counts = HashMap::with_capacity(8);
        for card in cards {
            *counts.entry(*card).or_insert(0) += 1;
        }
        counts
    }
}

impl From<&[u8; 5]> for Hand {
    fn from(value: &[u8; 5]) -> Self {
        let cards = array::from_fn(|i| Card::from(value[i]));
        Self::new(cards)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Bid {
    pub hand: Hand,
    pub value: u32,
}

pub fn load_input() -> Vec<Bid> {
    stdin()
        .lines()
        .map(|line| {
            let line = line.expect("failed to read from stdin");
            let (hand_str, value_str) = line.split_once(' ').unwrap();
            let hand_bytes = <&[u8; 5]>::try_from(hand_str.as_bytes()).unwrap();
            let hand = Hand::from(hand_bytes);
            let value = u32::from_str_radix(value_str, 10).unwrap();
            Bid { hand, value }
        })
        .collect()
}

fn main() {
    let mut bids = load_input();
    bids.sort_by_key(|bid| bid.hand);
    let result = bids
        .iter()
        .enumerate()
        .map(|(idx, bid)| (idx + 1) * bid.value as usize)
        .sum::<usize>();
    println!("{result}");
}
