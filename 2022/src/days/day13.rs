use aoc_2022::*;
use std::{
    cmp::Ordering,
    fs,
    io::{self, Read},
    str::FromStr,
};

// --- Day 13: Distress Signal ---
// You climb the hill and again try contacting the Elves. However, you instead
// receive a signal you weren't expecting: a distress signal.
//
// Your handheld device must still not be working properly; the packets from the
// distress signal got decoded out of order. You'll need to re-order the list
// of received packets (your puzzle input) to decode the message.
//
// Your list consists of pairs of packets; pairs are separated by a blank line.
// You need to identify how many pairs of packets are in the right order.
//
// Packet data consists of lists and integers. Each list starts with [, ends
// with ], and contains zero or more comma-separated values (either integers or
// other lists). Each packet is always a list and appears on its own line.
//
pub fn main() -> eyre::Result<()> {
    let input = fs::read_to_string("./inputs/13")?;

    println!("Day 13.");

    println!("Problem 1: {}", p1(&input)?);
    println!("Problem 2: {}", p2(&input)?);

    Ok(())
}

// -- Part One --
// What are the indices of the pairs that are already in the right order? (The
// first pair has index 1, the second pair has index 2, and so on.) In the
// above example, the pairs in the right order are 1, 2, 4, and 6; the sum of
// these indices is 13.
//
// Determine which pairs of packets are already in the right order. What is the
// sum of the indices of those pairs?
fn p1(input: &str) -> eyre::Result<usize> {
    Ok(input
        .split("\n\n")
        .enumerate()
        .map(|(i, s)| {
            let (l, r) = s.split_once('\n').res()?;
            let (l, r) = (l.parse()?, r.parse()?);
            Ok((i + 1, l, r))
        })
        .collect::<eyre::Result<Vec<(usize, Packet, Packet)>>>()?
        .into_iter()
        .filter(|(_i, l, r)| l <= r)
        .map(|(i, _l, _r)| i)
        .sum())
}

// -- Part Two ---
// Now, you just need to put all of the packets in the right order. Disregard
// the blank lines in your list of received packets.
//
// The distress signal protocol also requires that you include two additional
// divider packets:
//
// [[2]]
// [[6]]
//
// Using the same rules as before, organize all packets - the ones in your list
// of received packets as well as the two divider packets - into the correct order.
//
// For the example above, the result of putting the packets in the correct order is:
//
// []
// [[]]
// [[[]]]
// [1,1,3,1,1]
// [1,1,5,1,1]
// [[1],[2,3,4]]
// [1,[2,[3,[4,[5,6,0]]]],8,9]
// [1,[2,[3,[4,[5,6,7]]]],8,9]
// [[1],4]
// [[2]]
// [3]
// [[4,4],4,4]
// [[4,4],4,4,4]
// [[6]]
// [7,7,7]
// [7,7,7,7]
// [[8,7,6]]
// [9]
//
// Afterward, locate the divider packets. To find the decoder key for this
// distress signal, you need to determine the indices of the two divider packets
// and multiply them together. (The first packet is at index 1, the second
// packet is at index 2, and so on.) In this example, the divider packets are
// 10th and 14th, and so the decoder key is 140.
//
// Organize all of the packets into the correct order. What is the decoder key
// for the distress signal?
fn p2(input: &str) -> eyre::Result<usize> {
    const DIVIDER1: &str = "[[2]]";
    const DIVIDER2: &str = "[[6]]";

    let sentinels = [DIVIDER1, DIVIDER2];
    let divider_p = [DIVIDER1.parse().unwrap(), DIVIDER2.parse().unwrap()];

    let mut packets = input
        .lines()
        .filter(|l| !l.is_empty())
        .chain(sentinels)
        .map(|l| Ok(l.parse()?))
        .collect::<eyre::Result<Vec<Packet>>>()?;
    packets.sort();
    let results = packets
        .into_iter()
        .enumerate()
        .filter(|(_, p)| divider_p.contains(p))
        .map(|(i, _)| i + 1)
        .collect::<Vec<_>>();

    assert!(results.len() == 2);

    Ok(results.into_iter().fold(1, |a, b| a * b))
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    Single(usize),
    Array(Box<[Packet]>),
}

impl Packet {
    // fn is_single(&self) -> bool {
    //     matches!(self, Packet::Single(_))
    // }
    //
    // fn is_array(&self) -> bool {
    //     matches!(self, Packet::Array(_))
    // }
    //
    // fn as_single(&self) -> Option<&usize> {
    //     match self {
    //         Packet::Single(i) => Some(i),
    //         Packet::Array(_) => None,
    //     }
    // }
    //
    // fn as_array(&self) -> Option<&[Packet]> {
    //     match self {
    //         Packet::Array(d) => Some(&d),
    //         Packet::Single(_) => None,
    //     }
    // }
    //
    // fn wrap(self) -> Self {
    //     Self::Array(Box::new([self]))
    // }
}

impl FromStr for Packet {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if let Some(s) = s.strip_prefix('[') {
            let mut items = vec![];
            // method: ignor the brackets
            // read until you get to a comma
            // if the number of open brackets and closed brackets is the same: that is a token
            // if not then read to the next comma

            let s = s
                .strip_suffix(']')
                .expect("missing closing braket for opening");
            let len = s.len() as u64;

            let mut reader = io::Cursor::new(s);
            // skip the opening braket
            // reader.set_position(1);

            let mut buf = vec![];
            while reader.position() < len {
                reader.read_while(&mut buf, |buf| {
                    buf.iter().filter(|b| **b == b'[').count()
                        == buf.iter().filter(|b| **b == b']').count()
                })?;

                // remove the comma found or ending bracket
                // let _ = buf.pop();
                if buf.ends_with(&[b',']) {
                    buf.pop();
                    let s = String::from_utf8(buf.drain(..).collect())?;
                    // dbg!(&s);
                    items.push(s.parse()?);
                }
            }

            if !buf.is_empty() {
                let s = String::from_utf8(buf.drain(..).collect())?;
                // dbg!(&s);
                items.push(s.parse()?);
            }

            Ok(Self::Array(items.into_boxed_slice()))
        } else {
            Ok(Self::Single(s.parse()?))
        }
    }
}

// When comparing two values, the first value is called left and the second
// value is called right. Then:
//  - If both values are integers, the lower integer should come first. If the
//      left integer is lower than the right integer, the inputs are in the
//      right order. If the left integer is higher than the right integer, the
//      inputs are not in the right order. Otherwise, the inputs are the same
//      integer; continue checking the next part of the input.
//  - If both values are lists, compare the first value of each list, then the
//      second value, and so on. If the left list runs out of items first, the
//      inputs are in the right order. If the right list runs out of items
//      first, the inputs are not in the right order. If the lists are the same
//      length and no comparison makes a decision about the order, continue
//      checking the next part of the input.
//  - If exactly one value is an integer, convert the integer to a list which
//      contains that integer as its only value, then retry the comparison. For
//      example, if comparing [0,0,0] and 2, convert the right value to [2] (a
//      list containing 2); the result is then found by instead comparing
//      [0,0,0] and [2].
//
// Using these rules, you can determine which of the pairs in the example are
// in the right order:
//
// == Pair 1 ==
// - Compare [1,1,3,1,1] vs [1,1,5,1,1]
//   - Compare 1 vs 1
//   - Compare 1 vs 1
//   - Compare 3 vs 5
//     - Left side is smaller, so inputs are in the right order
//
// == Pair 2 ==
// - Compare [[1],[2,3,4]] vs [[1],4]
//   - Compare [1] vs [1]
//     - Compare 1 vs 1
//   - Compare [2,3,4] vs 4
//     - Mixed types; convert right to [4] and retry comparison
//     - Compare [2,3,4] vs [4]
//       - Compare 2 vs 4
//         - Left side is smaller, so inputs are in the right order
//
// == Pair 3 ==
// - Compare [9] vs [[8,7,6]]
//   - Compare 9 vs [8,7,6]
//     - Mixed types; convert left to [9] and retry comparison
//     - Compare [9] vs [8,7,6]
//       - Compare 9 vs 8
//         - Right side is smaller, so inputs are not in the right order
//
// == Pair 4 ==
// - Compare [[4,4],4,4] vs [[4,4],4,4,4]
//   - Compare [4,4] vs [4,4]
//     - Compare 4 vs 4
//     - Compare 4 vs 4
//   - Compare 4 vs 4
//   - Compare 4 vs 4
//   - Left side ran out of items, so inputs are in the right order
//
// == Pair 5 ==
// - Compare [7,7,7,7] vs [7,7,7]
//   - Compare 7 vs 7
//   - Compare 7 vs 7
//   - Compare 7 vs 7
//   - Right side ran out of items, so inputs are not in the right order
//
// == Pair 6 ==
// - Compare [] vs [3]
//   - Left side ran out of items, so inputs are in the right order
//
// == Pair 7 ==
// - Compare [[[]]] vs [[]]
//   - Compare [[]] vs []
//     - Right side ran out of items, so inputs are not in the right order
//
// == Pair 8 ==
// - Compare [1,[2,[3,[4,[5,6,7]]]],8,9] vs [1,[2,[3,[4,[5,6,0]]]],8,9]
//   - Compare 1 vs 1
//   - Compare [2,[3,[4,[5,6,7]]]] vs [2,[3,[4,[5,6,0]]]]
//     - Compare 2 vs 2
//     - Compare [3,[4,[5,6,7]]] vs [3,[4,[5,6,0]]]
//       - Compare 3 vs 3
//       - Compare [4,[5,6,7]] vs [4,[5,6,0]]
//         - Compare 4 vs 4
//         - Compare [5,6,7] vs [5,6,0]
//           - Compare 5 vs 5
//           - Compare 6 vs 6
//           - Compare 7 vs 0
//             - Right side is smaller, so inputs are not in the right order
//
impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Single(l), Self::Single(r)) => l.cmp(r),
            (Self::Array(l), Self::Array(r)) => {
                let mut left = l.into_iter();
                let mut right = r.into_iter();
                loop {
                    let l = left.next();
                    let r = right.next();

                    match (l, r) {
                        (None, None) => return Ordering::Equal,
                        (None, Some(_)) => return Ordering::Less,
                        (Some(_), None) => return Ordering::Greater,
                        (Some(l), Some(r)) => {
                            let ord = l.cmp(r);
                            if !ord.is_eq() {
                                return ord;
                            }
                        }
                    }
                }
            }

            (Self::Single(single), Self::Array(right)) => {
                let left = [Self::Single(*single)];
                Ord::cmp(&left[..], &right[..])
            }

            // the left will always be a list
            (Self::Array(left), Self::Single(single)) => {
                let right = [Self::Single(*single)];
                Ord::cmp(&left[..], &right[..])
            }
        }
    }
}
impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub trait ReadWhile: Read {
    fn read_while<F>(&mut self, buf: &mut Vec<u8>, f: F) -> io::Result<usize>
    where
        F: Fn(&[u8]) -> bool;
}

impl<R: Read> ReadWhile for R {
    fn read_while<F>(&mut self, buf: &mut Vec<u8>, f: F) -> io::Result<usize>
    where
        F: Fn(&[u8]) -> bool,
    {
        let mut read = 0;

        loop {
            let mut b = [0; 1];
            read += self.read(&mut b)?;
            let [b] = b;
            buf.push(b);

            if f(&buf) {
                return Ok(read);
            }
        }
    }
}
