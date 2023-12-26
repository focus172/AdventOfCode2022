use std::{fmt, str::FromStr};

use error_stack::{Context, Report, Result, ResultExt};

const NUMBER_OF_RACES: usize = 4;

#[derive(Debug)]
struct RaceInfo {
    time: [usize; NUMBER_OF_RACES],
    dist: [usize; NUMBER_OF_RACES],
}

#[derive(Debug)]
pub enum ParseRaceInfoError {
    FailedIdk,
}
impl fmt::Display for ParseRaceInfoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("coult not do it")
    }
}
impl Context for ParseRaceInfoError {}

impl FromStr for RaceInfo {
    type Err = Report<ParseRaceInfoError>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        // Time:        55     82     64     90
        // Distance:   246   1441   1012   1111
        let (time, dist) = s.split_once('\n').unwrap();
        let times: Vec<usize> = time
            .split_whitespace()
            .skip(1)
            .map(|i| i.trim().parse().unwrap())
            .collect();

        let dists: Vec<usize> = dist
            .split_whitespace()
            .skip(1)
            .map(|i| i.trim().parse().unwrap())
            .collect();

        let time = <[_; NUMBER_OF_RACES]>::default();
        let a = Self {
            time: a,
            dist: todo!(),
        }
        todo!()

        // RaceInfo_t ret;
        // std::string buf;
        // std::cin >> buf;
        //
        // std::cin >> ret.time[0];
        // std::cin >> ret.time[1];
        // std::cin >> ret.time[2];
        // std::cin >> ret.time[3];
        //
        // std::cin >> buf;
        //
        // std::cin >> ret.dist[0];
        // std::cin >> ret.dist[1];
        // std::cin >> ret.dist[2];
        // std::cin >> ret.dist[3];
        //
        // // std::cout << buf;
        // return ret;
    }
}

#[derive(Debug)]
pub struct AnyError;
impl fmt::Display for AnyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("generic error")
    }
}
impl Context for AnyError {}

fn main() -> Result<(), AnyError> {
    let input = include_str!("../../input/06");
    let ri = RaceInfo::from_str(input).change_context(AnyError)?;
    // race_info_debug(&ri);

    println!("Problem 1: {}", p1(&ri));
    // 23133440 is too high

    // (time - wait) * wait > dist
    // time * wait - wait ^ 2 > dist
    // 0 = wait^2 - time * wait + dist
    // wait = time + sqrt( time^2 - 4 * time ) / 2
    // uint32_t sum = 1;
    // for (int i = 0; i < NUMBER_OF_RACES; i++) {
    //   uint32_t wait = 3;
    //   uint32_t total = (ri.time[i] - wait) * wait;
    //   double t = std::sqrt((ri.time[i] - 4) * ri.time[i]);
    //   double b = (t + ri.time[i]) / 2.0;
    //   uint32_t up = b + 1;
    //   uint32_t span = ri.time[i] - 2 * up;
    //   sum *= span;
    // }
    // std::cout << sum;
    //
    // // TODO
    // return 0;
    Ok(())
}

fn p1(ri: &RaceInfo) -> usize {
    todo!()
}
