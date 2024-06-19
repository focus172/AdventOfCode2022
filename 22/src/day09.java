import java.nio.file.Files;
import java.nio.file.Paths;

class day09 {
    public static void main(String[] args) throws Exception {
        String data = readInput();
    }

    public static String readInput() throws Exception {
        return new String(Files.readAllBytes(Paths.get("./input/09")));
    }
}

//#![feature(yeet_expr)]
//
//use core::fmt;
//use error_stack::{Context, Report, Result, ResultExt};
//use glam::IVec2;
//use std::{collections::HashSet, str::FromStr};
//
//
//pub fn main() -> eyre::Result<()> {
//    const INPUT: &str = include_str!("../../input/09");
//
//    println!("Day 9.");
//
//    println!("Problem 1: {}", p1(INPUT).unwrap());
//    println!("Problem 2: {}", p2(INPUT)?);
//
//    Ok(())
//}
//
//fn p1(input: &str) -> Result<usize, Day9Error> {
//    let mut visited_map = HashSet::new();
//    let mut head = IVec2::new(0, 0);
//    let mut tail = IVec2::new(0, 0);
//
//    for command in input
//        .lines()
//        .map(FromStr::from_str)
//        .collect::<Result<Vec<Command>, ParseCommandError>>()
//        .change_context(Day9Error)
//        .attach_printable("Input is not a valid list of commands.")?
//    {
//        for _ in 0..command.repeat {
//            let temp = head;
//            head += command.direction;
//            if head.distance_squared(tail) > 2 {
//                tail = temp;
//            }
//            visited_map.insert(tail);
//        }
//    }
//    Ok(visited_map.len())
//}
//
//fn p2(_input: &str) -> eyre::Result<&str> {
//    Ok("todo!")
//}
//
//struct Command {
//    direction: IVec2,
//    repeat: usize,
//}
//
//#[derive(Debug)]
//struct ParseCommandError;
//
//impl fmt::Display for ParseCommandError {
//    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//        f.write_str("Failed to parse text as a command.")
//    }
//}
//
//impl Context for ParseCommandError {}
//
//impl FromStr for Command {
//    type Err = Report<ParseCommandError>;
//
//    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
//        let (direction, repeat) = s.split_once(' ').ok_or_else(|| {
//            Report::new(ParseCommandError).attach_printable(format!(
//                "Expected (exactly) two space sperated values in {s:?}."
//            ))
//        })?;
//
//        let direction = match direction {
//            "U" => IVec2 { x: 0, y: 1 },
//            "D" => IVec2::new(0, -1),
//            "R" => IVec2::new(1, 0),
//            "L" => IVec2::new(-1, 0),
//            _ => {
//                do yeet Report::new(ParseCommandError).attach_printable(format!(
//                    "Expected a direction indicator from [U, D, L, R] found {direction:?}."
//                ))
//            }
//        };
//
//        let repeat = repeat
//            .parse::<usize>()
//            .change_context(ParseCommandError)
//            .attach_printable_lazy(|| format!("failed to parse {repeat} as an usize."))?;
//
//        Ok(Command { direction, repeat })
//    }
//}
