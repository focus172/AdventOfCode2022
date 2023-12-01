#![feature(lazy_cell)]

use aoc_2022::*;
use regex::Regex;
use std::{collections::HashMap, str::FromStr, sync::LazyLock};

// --- Day 15: Beacon Exclusion Zone ---
//
// You feel the ground rumble again as the distress signal leads you to a large
// network of subterranean tunnels. You don't have time to search them all, but
// you don't need to: your pack contains a set of deployable sensors that you
// imagine were originally built to locate lost Elves.
//
// The sensors aren't very powerful, but that's okay; your handheld device
// indicates that you're close enough to the source of the distress signal to
// use them. You pull the emergency sensor system out of your pack, hit the big
// button on top, and the sensors zoom off down the tunnels.
//
// Once a sensor finds a spot it thinks will give it a good reading, it
// attaches itself to a hard surface and begins monitoring for the nearest
// signal source beacon. Sensors and beacons always exist at integer
// coordinates. Each sensor knows its own position and can determine the
// position of a beacon precisely; however, sensors can only lock on to the one
// beacon closest to the sensor as measured by the Manhattan distance. (There
// is never a tie where two beacons are the same distance to a sensor.)
//
// It doesn't take long for the sensors to report back their positions and
// closest beacons (your puzzle input).
//
// None of the detected beacons seem to be producing the distress signal, so
// you'll need to work out where the distress beacon is by working out where it
// isn't. For now, keep things simple by counting the positions where a beacon
// cannot possibly be along just a single row.
//
// Consult the report from the sensors you just deployed. In the row where
// y=2000000, how many positions cannot contain a beacon?
pub fn main() -> eyre::Result<()> {
    resu::install()?;

    let input = include_str!("../../input/15");

    println!("Day 15.");

    println!("Problem 1: {}", p1(input)?);
    println!("Problem 2: {}", p2(input)?);

    Ok(())
}

fn p1(input: &str) -> eyre::Result<usize> {
    const ROW: isize = 2_000_000;

    let mut spans: Vec<_> = input
        .lines()
        .map(Sensor::from_str)
        .collect::<eyre::Result<Vec<_>>>()?
        .into_iter()
        .flat_map(|sensor| {
            let (pos_x, pos_y) = sensor.pos;
            let (x2, y2) = sensor.beacon;

            let reach = (x2 - pos_x).abs() + (y2 - pos_y).abs();

            let y_dist = (ROW - pos_y).abs();

            if y_dist < reach {
                let x_dist = reach - y_dist;
                Some((pos_x - x_dist)..(pos_x + x_dist))
            } else {
                None
            }
        })
        .collect();

    spans.sort_by(|a, b| a.start.cmp(&b.start));

    let mut top_seen = isize::MIN;

    let count = spans.iter().fold(0, |count, s| {
        debug_assert!(s.start <= s.end);

        let bot = std::cmp::max(top_seen, s.start);
        let top = std::cmp::max(top_seen, s.end);
        top_seen = top;
        // println!("Counting from {} to {}", bot, top);
        count + (top - bot) as usize
    });

    Ok(count)
}

// --- Part Two ---
// Your handheld device indicates that the distress signal is coming from a
// beacon nearby. The distress beacon is not detected by any sensor, but the
// distress beacon must have x and y coordinates each no lower than 0 and no
// larger than 4000000.
//
// To isolate the distress beacon's signal, you need to determine its tuning
// frequency, which can be found by multiplying its x coordinate by 4000000 and
// then adding its y coordinate.
//
// In the example above, the search space is smaller: instead, the x and y
// coordinates can each be at most 20. With this reduced search area, there is
// only a single position that could have a beacon: x=14, y=11. The tuning
// frequency for this distress beacon is 56000011.
//
// Find the only possible position for the distress beacon. What is its tuning
// frequency?
fn p2(input: &str) -> eyre::Result<usize> {
    input
        .lines()
        .map(Sensor::from_str)
        .collect::<eyre::Result<Vec<_>>>()?
        .into_iter()
        .flat_map(|sensor| {
            let reach = sensor.reach();
            let (x, y) = sensor.pos;

            (0..reach)
                .flat_map(|x_dist| {
                    let y_dist = reach - x_dist;
                    vec![
                        (x - x_dist, (y - y_dist)..(y + y_dist)),
                        (x + x_dist, (y - y_dist)..(y + y_dist)),
                    ]
                })
                .collect::<Vec<_>>()
        })
        .fold(HashMap::<isize, Vec<_>>::new(), |mut cols, (col, range)| {
            if (0..=4_000_000).contains(&col) {
                return cols;
            }

            match cols.get_mut(&col) {
                Some(r) => r.push(range),
                None => _ = cols.insert(col, vec![range]),
            }
            cols
        })
        .into_iter()
        .flat_map(|(col, mut spans)| {
            spans.sort_by(|a, b| a.start.cmp(&b.start));

            let mut top_seen = 0;

            spans
                .iter()
                .flat_map(|s| {
                    debug_assert!(s.start <= s.end);

                    let bot = std::cmp::max(top_seen, s.start);
                    if bot > top_seen {
                        return Some((col, top_seen + 1));
                    }
                    let top = std::cmp::max(top_seen, s.end);
                    let top = std::cmp::min(top, 4_000_000);
                    top_seen = top;
                    // println!("Counting from {} to {}", bot, top);
                    None
                })
                .next()
        })
        .next()
        .map(|(x, y)| (x * 4_000_000 + y) as usize)
        .ok_or(eyre::eyre!("Could not find any point not covered."))
}

#[derive(Debug)]
struct Sensor {
    pos: (isize, isize),
    beacon: (isize, isize),
}

impl Sensor {
    #[inline]
    pub fn reach(&self) -> isize {
        let (pos_x, pos_y) = self.pos;
        let (x2, y2) = self.beacon;

        (x2 - pos_x).abs() + (y2 - pos_y).abs()
    }
}

const __SENSOR_REGEX_INNER: &str = r"Sensor at x=(.+), y=(.+): closest beacon is at x=(.+), y=(.+)";
static SENSOR_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(__SENSOR_REGEX_INNER).unwrap());

impl FromStr for Sensor {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // static DIGET_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\d+").unwrap());
        // let mut caps = DIGET_REGEX.captures_iter(s);
        // let sensor_x = caps.next().res()?[0].parse()?;
        // let sensor_y = caps.next().res()?[0].parse()?;
        // let beacon_x = caps.next().res()?[0].parse()?;
        // let beacon_y = caps.next().res()?[0].parse()?;

        let caps = SENSOR_REGEX.captures(s).res()?;
        let sensor_x = caps[1].parse()?;
        let sensor_y = caps[2].parse()?;
        let beacon_x = caps[3].parse()?;
        let beacon_y = caps[4].parse()?;

        // std::hint::spin_loop();
        Ok(Self {
            pos: (sensor_x, sensor_y),
            beacon: (beacon_x, beacon_y),
        })
    }
}
