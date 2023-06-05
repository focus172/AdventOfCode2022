use std::{collections::HashSet, fs};

const DAY_HEADER: &str = "
██████╗  █████╗ ██╗   ██╗     ██████╗  █████╗ 
██╔══██╗██╔══██╗╚██╗ ██╔╝    ██╔═████╗██╔══██╗
██║  ██║███████║ ╚████╔╝     ██║██╔██║╚██████║
██║  ██║██╔══██║  ╚██╔╝      ████╔╝██║ ╚═══██║
██████╔╝██║  ██║   ██║       ╚██████╔╝ █████╔╝
╚═════╝ ╚═╝  ╚═╝   ╚═╝        ╚═════╝  ╚════╝ 
";

pub fn main() {
    println!("{}", DAY_HEADER);
    let raw = fs::read_to_string("/home/focus/code/AdventOfCode2022/inputs/9").expect("File does not exist");
    problem1(&raw);
    problem2(&raw);
}

fn problem1(raw: &str) {
    let mut visited_map = HashSet::new();
    let mut head = Point::new(0, 0);
    let mut tail = Point::new(0, 0);
    raw.lines().for_each(|line| {
        let split = line.split(" ").collect::<Vec<&str>>();
        let transform: Point = parseDirection(split[0]);
        let distance = split[1].parse::<i32>().unwrap(); 

        (1..=distance).for_each(|_| {
            head.transform(transform.x, transform.y);
            if head.distance(&tail) >= 1.9 {
                tail.transform(transform.x, transform.y);
            }
            let hash = format!("{}:{}", tail.x, tail.y);
            visited_map.insert(hash);
        });
    });
    println!("Problem 1: {}", visited_map.len());
}

fn problem2(raw: &str) {
    todo!();
}

fn parseDirection(letter: &str) -> Point {
    match letter {
        "U" => { Point::new(0, 1) }
        "D" => { Point::new(0, 1) }
        "R" => { Point::new(1, 0) }
        "L" => { Point::new(-1, 0 ) }
        _ => { panic!("Invalid direction") }
    }
}

#[derive(Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn transform(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }

    fn distance(&self, other: &Point) -> f32 {
        let x = (self.x - other.x).pow(2);
        let y = (self.y - other.y).pow(2);
        ((x + y) as f32).sqrt()
    }
}
