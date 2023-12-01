use aoc_2022::*;
use std::ops::{AddAssign, Sub, SubAssign};
use std::{collections::HashSet, str::FromStr};

const __DAY_HEADER: &str = "
██████╗  █████╗ ██╗   ██╗     ██████╗  █████╗
██╔══██╗██╔══██╗╚██╗ ██╔╝    ██╔═████╗██╔══██╗
██║  ██║███████║ ╚████╔╝     ██║██╔██║╚██████║
██║  ██║██╔══██║  ╚██╔╝      ████╔╝██║ ╚═══██║
██████╔╝██║  ██║   ██║       ╚██████╔╝ █████╔╝
╚═════╝ ╚═╝  ╚═╝   ╚═╝        ╚═════╝  ╚════╝
";

pub fn main() -> eyre::Result<()> {
    println!("Day 9.");

    let input = include_str!("../../input/09");

    println!("Problem 1: {}", p1(input)?);
    println!("Problem 2: {}", p2(input)?);

    Ok(())
}

use std::f32::consts::SQRT_2;

fn p1(input: &str) -> eyre::Result<usize> {
    let mut visited_map = HashSet::new();
    let mut head = Point::new(0, 0);
    let mut tail = Point::new(0, 0);

    for line in input.lines() {
        let split = line.split(' ').collect::<Vec<&str>>();
        let transform: Point = split[0].parse().unwrap();
        let repititions = split[1].parse::<i32>().unwrap();

        for _ in 0..repititions {
            let temp = head;
            head += transform;
            if head.distance(&tail) > SQRT_2 {
                // tail = head - transform;
                tail = temp;
            }
            visited_map.insert(tail);
        }
    }
    Ok(visited_map.len())
}

fn p2(_input: &str) -> eyre::Result<&str> {
    Ok("todo!")
}

impl FromStr for Point {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Point { x: 0, y: 1 }),
            "D" => Ok(Point::new(0, -1)),
            "R" => Ok(Point::new(1, 0)),
            "L" => Ok(Point::new(-1, 0)),
            _ => Err(eyre::eyre!("unknown ident")),
        }
    }
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
impl Sub for Point {
    type Output = Point;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn distance(&self, other: &Point) -> f32 {
        let x = (self.x - other.x).pow(2);
        let y = (self.y - other.y).pow(2);
        ((x + y) as f32).sqrt()
    }
}
