extern crate aoc_2023;
use aoc_2023::*;

// --- Day 1: Trebuchet?! ---
//
// Something is wrong with global snow production, and you've been selected to take a look. The Elves have even given you a map; on it, they've used stars to mark the top fifty locations that are likely to be having problems.
//
// You've been doing this long enough to know that to restore snow operations, you need to check all fifty stars by December 25th.
//
// Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!
//
// You try to ask why they can't just use a weather machine ("not powerful enough") and where they're even sending you ("the sky") and why your map looks mostly blank ("you sure ask a lot of questions") and hang on did you just say the sky ("of course, where do you think snow comes from") when you realize that the Elves are already loading you into a trebuchet ("please hold still, we need to strap you in").
//
// As they're making the final adjustments, they discover that their calibration document (your puzzle input) has been amended by a very young Elf who was apparently just excited to show off her art skills. Consequently, the Elves are having trouble reading the values on the document.
//
// The newly-improved calibration document consists of lines of text; each line originally contained a specific calibration value that the Elves now need to recover. On each line, the calibration value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.
//
// For example:
//
// 1abc2
// pqr3stu8vwx
// a1b2c3d4e5f
// treb7uchet
//
// In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.
//
// Consider your entire calibration document. What is the sum of all of the calibration values?
fn main() -> Result<()> {
    let input = include_str!("../../input/01");

    println!("Day 01.");

    println!("Problem 1: {}", p1(input)?);
    println!("Problem 1: {}", p2(input)?);

    Ok(())
}

fn p1(input: &str) -> Result<usize> {
    Ok(input
        .lines()
        .map(|s| {
            let mut iter = s.chars().filter(|c| c.is_ascii_digit());
            let first = iter.next().unwrap();
            let last = iter.last().unwrap_or(first);
            first.to_digit(10).unwrap() * 10 + last.to_digit(10).unwrap()
        })
        .sum::<u32>() as usize)
}

// --- Part Two ---
//
// Your calculation isn't quite right. It looks like some of the digits are
// actually spelled out with letters: one, two, three, four, five, six, seven,
// eight, and nine also count as valid "digits".
//
// Equipped with this new information, you now need to find the real first and
// last digit on each line. For example:
//
// two1nine
// eightwothree
// abcone2threexyz
// xtwone3four
// 4nineeightseven2
// zoneight234
// 7pqrstsixteen
//
// In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76. Adding these together produces 281.
//
// What is the sum of all of the calibration values?
fn p2(input: &str) -> Result<usize> {
    Ok(input
        .lines()
        .map(|s| {
            let mut iter = (0..s.len()).flat_map(|i| read_number_start(&s[i..]));

            let first = iter.next().unwrap();
            let last = iter.last().unwrap_or(first);
            // println!("Got values {first} and {last} from string {s}");
            first * 10 + last
        })
        .sum())
}

fn read_number_start(input: &str) -> Option<usize> {
    match input {
        i if i.starts_with('1') => Some(1),
        i if i.starts_with('2') => Some(2),
        i if i.starts_with('3') => Some(3),
        i if i.starts_with('4') => Some(4),
        i if i.starts_with('5') => Some(5),
        i if i.starts_with('6') => Some(6),
        i if i.starts_with('7') => Some(7),
        i if i.starts_with('8') => Some(8),
        i if i.starts_with('9') => Some(9),
        i if i.starts_with("one") => Some(1),
        i if i.starts_with("two") => Some(2),
        i if i.starts_with("three") => Some(3),
        i if i.starts_with("four") => Some(4),
        i if i.starts_with("five") => Some(5),
        i if i.starts_with("six") => Some(6),
        i if i.starts_with("seven") => Some(7),
        i if i.starts_with("eight") => Some(8),
        i if i.starts_with("nine") => Some(9),
        _ => None,
    }
}
