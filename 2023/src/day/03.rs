use std::{
    collections::HashMap,
    rc::Rc,
    sync::atomic::{AtomicBool, Ordering},
};

use aoc_2023::*;

fn main() -> Result<()> {
    let input = include_str!("../../input/03");

    println!("Day 03.");

    println!("Problem 1: {}", p1(input)?);
    println!("Problem 2: {}", p2(input)?); // 65804563 is too low

    Ok(())
}

#[derive(Debug)]
struct Number {
    value: usize,
    valid: Rc<AtomicBool>,
}

impl Number {
    fn new(value: usize) -> Self {
        Number {
            value,
            valid: Rc::new(AtomicBool::new(false)),
        }
    }
}

fn p1(input: &str) -> Result<usize> {
    let mut numbers: Vec<Number> = vec![];
    let mut points: HashMap<(isize, isize), Rc<AtomicBool>> = HashMap::new();

    for (line, y) in input.lines().zip(0isize..) {
        let mut read_span = None;
        // value is chained with non number so buffer is always flushed on last iteration
        for (x, c) in line.chars().chain(Some('B')).enumerate() {
            if c.is_ascii_digit() {
                let span = read_span.get_or_insert_with(|| x..x);
                span.end += 1;
            } else if let Some(span) = read_span.take() {
                let value = line[span.clone()].parse()?;

                let num = Number::new(value);
                for x in span {
                    let valid = num.valid.clone();
                    points.insert((x as isize, y), valid);
                }
                numbers.push(num);
            }
        }
    }

    input.lines().zip(0isize..).for_each(|(line, y)| {
        line.char_indices()
            .filter_map(|(x, c)| is_punct(&c).then_some(x as isize))
            .for_each(|x| {
                let update_bool = |b: &Rc<AtomicBool>| b.store(true, Ordering::Relaxed);

                points.get(&(x - 1, y - 1)).map(update_bool);
                points.get(&(x - 1, y)).map(update_bool);
                points.get(&(x - 1, y + 1)).map(update_bool);
                points.get(&(x, y - 1)).map(update_bool);
                points.get(&(x, y + 1)).map(update_bool);
                points.get(&(x + 1, y - 1)).map(update_bool);
                points.get(&(x + 1, y)).map(update_bool);
                points.get(&(x + 1, y + 1)).map(update_bool);
            });
    });

    // dbg!(&points);

    Ok(numbers
        .into_iter()
        .filter_map(|n| n.valid.load(Ordering::Relaxed).then_some(n.value))
        .sum())
}

fn p2(input: &str) -> Result<usize> {
    let mut points: HashMap<(isize, isize), Rc<Number>> = HashMap::new();

    for (line, y) in input.lines().zip(0isize..) {
        let mut read_span = None;
        // value is chained with non number so buffer is always flushed on last iteration
        for (x, c) in line.chars().chain(Some('B')).enumerate() {
            if c.is_ascii_digit() {
                let span = read_span.get_or_insert_with(|| x..x);
                span.end += 1;
            } else if let Some(span) = read_span.take() {
                let value = line[span.clone()].parse()?;

                let num = Rc::new(Number::new(value));
                for x in span {
                    let n = num.clone();
                    points.insert((x as isize, y), n);
                }
            }
        }
    }

    Ok(input
        .lines()
        .zip(0isize..)
        .map(|(line, y)| {
            line.char_indices()
                .filter_map(|(x, c)| c.eq(&'*').then_some(x as isize))
                .map(|x| {
                    let v = vec![
                        points.get(&(x - 1, y - 1)),
                        points.get(&(x - 1, y)),
                        points.get(&(x - 1, y + 1)),
                        points.get(&(x, y - 1)),
                        points.get(&(x, y + 1)),
                        points.get(&(x + 1, y - 1)),
                        points.get(&(x + 1, y)),
                        points.get(&(x + 1, y + 1)),
                    ];
                    let mut u = vec![];
                    let v: Vec<_> = v
                        .into_iter()
                        .flatten()
                        .filter_map(|n| {
                            let p = n.valid.as_ptr();
                            if !u.contains(&p) {
                                u.push(p);
                                Some(n.value)
                            } else {
                                None
                            }
                        })
                        .collect();

                    if v.len() == 2 {
                        v.into_iter().product()
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum())
}

fn is_punct(c: &char) -> bool {
    !c.is_ascii_digit() && *c != '.'
}
