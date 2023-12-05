#![feature(array_windows)]

use std::{ops::Range, str::FromStr};

use aoc_2023::*;

const INPUT: &str = include_str!("../../input/05");

fn main() -> Result<()> {
    println!("Day 05.");

    let almanac: Almanac = INPUT.parse()?;

    println!("Problem 1: {}", p1(&almanac));
    println!("Problem 2: {}", p2(&almanac));

    Ok(())
}

// --- Day 5: If You Give A Seed A Fertilizer ---
//
// You take the boat and find the gardener right where you were told he would be: managing a giant
// "garden" that looks more to you like a farm.
//
// "A water source? Island Island is the water source!" You point out that Snow Island isn't receiving any water.
//
// "Oh, we had to stop the water because we ran out of sand to filter it with! Can't make snow with
// dirty water. Don't worry, I'm sure we'll get more sand soon; we only turned off the water a few
// days... weeks... oh no." His face sinks into a look of horrified realization.
//
// "I've been so busy making sure everyone here has food that I completely forgot to check why we stopped getting
// more sand! There's a ferry leaving soon that is headed over in that direction - it's much faster than your
// boat. Could you please go check it out?"
//
// You barely have time to agree to this request when he brings up another. "While you wait for the ferry,
// maybe you can help us with our food production problem. The latest Island Island Almanac just arrived
// and we're having trouble making sense of it."
//
// The almanac (your puzzle input) lists all of the seeds that need to be
// planted. It also lists what type of soil to use with each kind of seed,
// what type of fertilizer to use with each kind of soil, what type of
// water to use with each kind of fertilizer, and so on. Every type of
// seed, soil, fertilizer and so on is identified with a number, but
// numbers are reused by each category - that is, soil 123 and fertilizer
// 123 aren't necessarily related to each other.
//
// For example:
//
// seeds: 79 14 55 13
//
// seed-to-soil map:
// 50 98 2
// 52 50 48
//
// soil-to-fertilizer map:
// 0 15 37
// 37 52 2
// 39 0 15
//
// fertilizer-to-water map:
// 49 53 8
// 0 11 42
// 42 0 7
// 57 7 4
//
// water-to-light map:
// 88 18 7
// 18 25 70
//
// light-to-temperature map:
// 45 77 23
// 81 45 19
// 68 64 13
//
// temperature-to-humidity map:
// 0 69 1
// 1 0 69
//
// humidity-to-location map:
// 60 56 37
// 56 93 4
//
// The almanac starts by listing which seeds need to be planted: seeds 79, 14, 55, and 13.
//
// The rest of the almanac contains a list of maps which describe how to
// convert numbers from a source category into numbers in a destination
// category. That is, the section that starts with seed-to-soil map:
// describes how to convert a seed number (the source) to a soil number
// (the destination). This lets the gardener and his team know which soil
// to use with which seeds, which water to use with which fertilizer, and so on.
//
// Rather than list every source number and its corresponding destination
// number one by one, the maps describe entire ranges of numbers that can
// be converted. Each line within a map contains three numbers: the destination
// range start, the source range start, and the range length.
//
// Consider again the example seed-to-soil map:
//
// 50 98 2
// 52 50 48
//
// The first line has a destination range start of 50, a source range start of
// 98, and a range length of 2. This line means that the source range starts at
// 98 and contains two values: 98 and 99. The destination range is the same
// length, but it starts at 50, so its two values are 50 and 51. With this
// information, you know that seed number 98 corresponds to soil number 50
// and that seed number 99 corresponds to soil number 51.
//
// The second line means that the source range starts at 50 and contains 48 values: 50, 51, ..., 96, 97.
// This corresponds to a destination range starting at 52 and also containing 48 values: 52, 53, ..., 98,
// 99. So, seed number 53 corresponds to soil number 55.
//
// Any source numbers that aren't mapped correspond to the same destination number. So, seed number 10
// corresponds to soil number 10.
//
// So, the entire list of seed numbers and their corresponding soil numbers looks like this:
//
// seed  soil
// 0     0
// 1     1
// ...   ...
// 48    48
// 49    49
// 50    52
// 51    53
// ...   ...
// 96    98
// 97    99
// 98    50
// 99    51
//
// With this map, you can look up the soil number required for each initial seed number:
//
//     Seed number 79 corresponds to soil number 81.
//     Seed number 14 corresponds to soil number 14.
//     Seed number 55 corresponds to soil number 57.
//     Seed number 13 corresponds to soil number 13.
//
// The gardener and his team want to get started as soon as possible, so they'd like to know the closest location
// that needs a seed. Using these maps, find the lowest location number that corresponds to any of the initial
// seeds. To do this, you'll need to convert each seed number through other categories until you can find its
// corresponding location number. In this example, the corresponding types are:
//
//     Seed 79, soil 81, fertilizer 81, water 81, light 74, temperature 78, humidity 78, location 82.
//     Seed 14, soil 14, fertilizer 53, water 49, light 42, temperature 42, humidity 43, location 43.
//     Seed 55, soil 57, fertilizer 57, water 53, light 46, temperature 82, humidity 82, location 86.
//     Seed 13, soil 13, fertilizer 52, water 41, light 34, temperature 34, humidity 35, location 35.
//
// So, the lowest location number in this example is 35.
//
// What is the lowest location number that corresponds to any of the initial seed numbers?
fn p1(almanac: &Almanac) -> usize {
    almanac
        .seeds
        .iter()
        .map(|s| almanac.seed_to_soil.get(s))
        .map(|s| almanac.soil_to_fert.get(&s))
        .map(|s| almanac.fert_to_watr.get(&s))
        .map(|s| almanac.watr_to_lght.get(&s))
        .map(|s| almanac.lght_to_temp.get(&s))
        .map(|s| almanac.temp_to_humd.get(&s))
        .map(|s| almanac.humd_to_loct.get(&s))
        .min()
        .unwrap()
}

fn p2(almanac: &Almanac) -> usize {
    almanac
        .seeds
        .array_windows()
        .step_by(2)
        .flat_map(|[start, len]| *start..start + len)
        // .flat_map(|range| {
        //     dbg!(&range);
        //     range
        // })
        .map(|s| almanac.seed_to_soil.get(&s))
        .map(|s| almanac.soil_to_fert.get(&s))
        .map(|s| almanac.fert_to_watr.get(&s))
        .map(|s| almanac.watr_to_lght.get(&s))
        .map(|s| almanac.lght_to_temp.get(&s))
        .map(|s| almanac.temp_to_humd.get(&s))
        .map(|s| almanac.humd_to_loct.get(&s))
        .min()
        .unwrap()
}

#[derive(Debug)]
struct Almanac {
    pub seeds: Vec<usize>,
    seed_to_soil: Ranges,
    soil_to_fert: Ranges,
    fert_to_watr: Ranges,
    watr_to_lght: Ranges,
    lght_to_temp: Ranges,
    temp_to_humd: Ranges,
    humd_to_loct: Ranges,
}

impl FromStr for Almanac {
    type Err = eyre::Report;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut iter = s.split("\n\n");
        let seeds = iter
            .next()
            .res()?
            .trim()
            .strip_prefix("seeds: ")
            .res()?
            .split_whitespace()
            .map(|s| s.parse().map_err(Into::into))
            .collect::<Result<_>>()?;
        let seed_to_soil = iter.next().res()?.parse()?;
        let soil_to_fert = iter.next().res()?.parse()?;
        let fert_to_watr = iter.next().res()?.parse()?;
        let watr_to_lght = iter.next().res()?.parse()?;
        let lght_to_temp = iter.next().res()?.parse()?;
        let temp_to_humd = iter.next().res()?.parse()?;
        let humd_to_loct = iter.next().res()?.parse()?;

        debug_assert_eq!(iter.next(), None, "Unknown tokens at end of iter");

        Ok(Self {
            seeds,
            seed_to_soil,
            soil_to_fert,
            fert_to_watr,
            watr_to_lght,
            lght_to_temp,
            temp_to_humd,
            humd_to_loct,
        })
    }
}

#[derive(Debug)]
struct Ranges {
    spans: Vec<RangeMap>,
}

impl FromStr for Ranges {
    type Err = eyre::Report;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Ranges {
            spans: s
                .lines()
                .skip(1)
                .map(|l| {
                    let mut iter = l.split_whitespace().map(|e| e.parse());
                    let dst = iter.next().res()??;
                    let src = iter.next().res()??;
                    let len = iter.next().res()??;
                    Ok(RangeMap::new(src, dst, len))
                })
                .collect::<Result<_>>()?,
        })
    }
}

impl Ranges {
    pub fn get(&self, index: &usize) -> usize {
        self.spans
            .iter()
            .find_map(|range| range.get(index))
            .unwrap_or(*index)
    }
}

#[derive(Debug)]
struct RangeMap {
    src: Range<usize>,
    dst: Range<usize>,
}

impl RangeMap {
    pub fn new(src: usize, dst: usize, len: usize) -> Self {
        Self {
            src: src..src + len,
            dst: dst..dst + len,
        }
    }

    pub fn get(&self, value: &usize) -> Option<usize> {
        if self.src.contains(value) {
            let offset = value - self.src.start;
            Some(self.dst.start + offset)
        } else {
            None
        }
    }
}
