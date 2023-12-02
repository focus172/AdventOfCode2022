#![feature(lazy_cell)]

use aoc_2023::*;
use std::str::FromStr;

fn main() -> Result<()> {
    let input = include_str!("../../input/02");

    println!("Day 02.");

    println!("Problem 1: {}", p1(input)?); // 2886 is too high
    println!("Problem 2: {}", p2(input)?);

    Ok(())
}

const MAX_RED: usize = 12;
const MAX_GREEN: usize = 13;
const MAX_BLUE: usize = 14;

fn p1(input: &str) -> Result<usize> {
    Ok(input
        .lines()
        .map(Game::from_str)
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .map(|game| (game.number, game.get_max()))
        .filter(|(_, r)| r.red <= MAX_RED && r.green <= MAX_GREEN && r.blue <= MAX_BLUE)
        .map(|(number, _)| number)
        .sum::<usize>())
}

fn p2(input: &str) -> Result<usize> {
    Ok(input
        .lines()
        .map(Game::from_str)
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .map(|game| (game.number, game.get_max()))
        .map(|(_, max)| max.red * max.green * max.blue)
        .sum::<usize>())
}

struct Game {
    number: usize,
    rounds: Vec<Round>,
}

impl Game {
    fn get_max(&self) -> Round {
        self.rounds
            .iter()
            .fold(Round::default(), |mut maxes, round| {
                maxes.red = std::cmp::max(maxes.red, round.red);
                maxes.green = std::cmp::max(maxes.green, round.green);
                maxes.blue = std::cmp::max(maxes.blue, round.blue);
                maxes
            })
    }
}

impl FromStr for Game {
    type Err = resu::Report;

    // Exaplme input:
    // Game 99: 1 blue, 2 green, 2 red; 2 red, 8 green; 14 green, 1 blue; 1 red, 2 green; 1 blue, 1 green, 2 red; 6 green, 2 red
    fn from_str(s: &str) -> Result<Self> {
        let (front, back) = s.split_once(':').res()?;
        let number = front.trim_start_matches("Game ").parse::<usize>()?;

        let rounds = back
            .split(';')
            .map(Round::from_str)
            .collect::<Result<_>>()?;

        Ok(Self { number, rounds })
    }
}

#[derive(Debug, Default)]
struct Round {
    red: usize,
    green: usize,
    blue: usize,
}

impl FromStr for Round {
    type Err = resu::Report;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let mut ret = Self::default();

        for dice in s.split(',') {
            let dice = dice.trim();
            let (number, name) = dice.split_once(' ').res()?;
            let count = number.parse::<usize>()?;
            if name.contains("red") {
                ret.red = count;
            }
            if name.contains("green") {
                ret.green = count;
            }
            if name.contains("blue") {
                ret.blue = count;
            }
        }
        Ok(ret)
    }
}
