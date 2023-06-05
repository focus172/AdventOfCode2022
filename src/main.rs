#![allow(non_snake_case)]

use std::error::Error;

#[path = "days/Day09.rs"] mod day09;
#[path = "days/Day10.rs"] mod day10;

fn main() -> Result<(), Box<dyn Error>> {
    //day01::main();
    //day02::main();
    //day03::main();
    //day04::main();
    //day05::main();
    //day06::main();
    //day07::main();
    //day08::main();
    day09::main();
    day10::main()?;
    //day11::main();
    //day12::main();
    //day13::main();
    //day14::main();
    //day15::main();
    //day16::main(); 
    //day17::main();
    //day18::main();
    //day19::main();
    //day20::main();
    //day21::main();
    //day22::main();
    //day23::main();
    //day24::main();
    //day25::main();
    Ok(())
}
