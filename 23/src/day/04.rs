#![feature(inline_const)]
use std::{fmt, str::FromStr};

use aoc_2023::*;

fn main() -> Result<()> {
    const INPUT: &str = include_str!("../../input/04");
    let cards = INPUT
        .lines()
        .map(Card::from_str)
        .collect::<Result<Vec<_>>>()?;

    println!("Day 04.");

    println!("Problem 1: {}", p1(&cards));
    println!("Problem 2: {}", p2(&cards));

    Ok(())
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Game {}: {:?} | {:?}",
            self.round, self.winning, self.values
        )
    }
}

fn p1(cards: &[Card]) -> usize {
    cards
        .iter()
        .map(|card| card.winners().count() as u32)
        .map(|m| if m == 0 { 0 } else { 2usize.pow(m - 1) })
        .sum()
}

fn p2(cards: &[Card]) -> usize {
    let mut copies: Vec<_> = std::iter::repeat(1).take(cards.len()).collect();

    cards.iter().for_each(|card| {
        let matches = card.winners().count();
        let repeat = copies[card.round - 1];
        for j in (card.round)..(card.round + matches) {
            if let Some(c) = copies.get_mut(j) {
                *c += repeat;
            }
        }
    });

    copies.into_iter().sum()
}

#[derive(Debug)]
struct Card {
    round: usize,
    values: Vec<usize>,
    winning: Vec<usize>,
}

impl Card {
    /// Returns the winning values of this card
    pub fn winners(&self) -> impl Iterator<Item = usize> + '_ {
        self.values
            .clone()
            .into_iter()
            .filter(|v| self.winning.contains(v))
    }
}

impl FromStr for Card {
    type Err = eyre::Report;

    // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (game, rest) = s.split_once(':').res()?;
        let round = game.split_ascii_whitespace().last().res()?.parse()?;

        let (winning_names, values_names) = rest.split_once('|').res()?;

        let winning = winning_names
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        let values = values_names
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        Ok(Self {
            round,
            values,
            winning,
        })
    }
}
