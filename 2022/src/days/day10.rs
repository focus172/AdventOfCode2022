use std::fs;
use aoc_2022::*;

// https://textkool.com/en/test-ascii-art-generator
const __DAY_HEADER: &str = "
██████╗  █████╗ ██╗   ██╗     ██╗ ██████╗
██╔══██╗██╔══██╗╚██╗ ██╔╝    ███║██╔═████╗
██║  ██║███████║ ╚████╔╝     ╚██║██║██╔██║
██║  ██║██╔══██║  ╚██╔╝       ██║████╔╝██║
██████╔╝██║  ██║   ██║        ██║╚██████╔╝
╚═════╝ ╚═╝  ╚═╝   ╚═╝        ╚═╝ ╚═════╝
";

pub fn main() -> eyre::Result<()> {
    let input = fs::read_to_string("./inputs/10")?;

    // gets the answer to the second problem

    println!("Day 10.");

    // outputs the answers to IO
    println!("Problem 1: {}", p1(&input)?);
    println!("Problem 2: {}", p2(&input)?);

    Ok(())
}

// this code is responsible for keeping track on one counter as it runs through the input
// each input can either change the counter by some value or do nothing
// at the 20th operation and every 40th operation after that the counters value is multiplied by the number of operations add added to a total
// the total is then returned
fn p1(input: &String) -> eyre::Result<isize> {
    let mut total = 0;

    let mut registerx = 1;
    let mut cycle = 1;

    // iterates through the input counting each one as a operation
    // operations that take additional time are handled withing the loop
    for line in input.lines() {
        // check at the start of a cycle if the counter is valid and increment it
        total += score_register(cycle, registerx);

        // parse the operation into an instruction
        let mut split = line.split_whitespace();
        let instruction = split.next().unwrap();

        // if the instruction is noop then just add one to the counter
        if instruction == "addx" {
            // to repersent processing time for the operation
            // we can just add one to the counter now
            // this has the same effect as running the operation with noop
            // while it processes
            cycle += 1; // TLDR: wait 1

            total += score_register(cycle, registerx);

            // gets the value to add to the counter
            let value: isize = split.next().res()?.parse()?;
            registerx += value;

            cycle += 1; // TLDR: wait 1
        } else {
            // if the instruction is not noop then it is a noop
            // so we just add one to the counter
            cycle += 1;
        }
    }

    Ok(total)
}

fn score_register(cycle: isize, registerx: isize) -> isize {
    if (cycle + 20) % 40 == 0 {
        registerx * cycle
    } else {
        0
    }
}

const SCREEN_WIDTH: isize = 40;
const START_CYCLE: isize = 0;

// @dev builds an output array that mimics the way a crt works
// @param input a list of instructions that are either a noop or a addx
fn p2(input: &str) -> eyre::Result<String> {
    // the output array that will be built through the process
    let mut screen_output = Vec::<String>::new();

    let mut registerx = 1;
    let mut cycle = START_CYCLE;

    let mut current_row = String::new();

    // explian with this needs to exist for the code to function as intended
    render_pixel(&mut current_row, registerx, cycle, &mut screen_output);

    for line in input.lines() {
        // parse the operation into an instruction
        let mut split = line.split_whitespace();
        let instruction = split.next().unwrap();

        cycle += 1;
        // parsing the instructions
        match instruction {
            "addx" => {
                render_pixel(&mut current_row, registerx, cycle, &mut screen_output);

                let value: isize = split.next().res()?.parse()?;
                registerx += value;

                cycle += 1;
                render_pixel(&mut current_row, registerx, cycle, &mut screen_output);
            }
            "noop" => render_pixel(&mut current_row, registerx, cycle, &mut screen_output),
            _ => eyre::bail!("Error: invalid instruction"),
        }
    }

    Ok(screen_output.join("\n").to_owned())
}

fn render_pixel(
    current_row: &mut String,
    registerx: isize,
    cycle: isize,
    screen_output: &mut Vec<String>,
) {
    // the value of the register is the x position of the pixie
    // the pixie is 3 pixels wide and occupies the entire hieght of the screen
    // the cycle is what pixel is being rendered on the screen
    // each row has 'screen_width' pixels and so going over that number will wrap around to the next row

    // if the current row is full then add it to the output array
    // and reset the current row
    if cycle % SCREEN_WIDTH == START_CYCLE {
        screen_output.push(current_row.clone());
        current_row.clear();
    }

    // if the pixie is within the current row then add "#" to the current row
    // if not then add "." to the current row
    if registerx - 1 <= cycle % SCREEN_WIDTH && cycle % SCREEN_WIDTH <= registerx + 1 {
        current_row.push_str("#");
    } else {
        current_row.push_str(".");
    }
}
