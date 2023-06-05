use std::fs;
use std::io::Error;

pub fn main() -> Result<(), Error> {
    //https://textkool.com/en/test-ascii-art-generator

    println!("
██████╗  █████╗ ██╗   ██╗     ██╗ ██████╗ 
██╔══██╗██╔══██╗╚██╗ ██╔╝    ███║██╔═████╗
██║  ██║███████║ ╚████╔╝     ╚██║██║██╔██║
██║  ██║██╔══██║  ╚██╔╝       ██║████╔╝██║
██████╔╝██║  ██║   ██║        ██║╚██████╔╝
╚═════╝ ╚═╝  ╚═╝   ╚═╝        ╚═╝ ╚═════╝ 
    ");


    let input = fs::read_to_string("./inputs/Day10Input.txt")?;

    // gets the answer to the first problem
    let answer1 = problem1(&input);

    // gets the answer to the second problem
    let answer2 = problem2(&input);
    
    // outputs the answers to IO
    println!("Problem 1: {}", answer1);
    println!("Problem 2:");
    for line in answer2 {
        println!("{}", line);
    }

    Ok(())
}

// this code is responsible for keeping track on one counter as it runs through the input
// each input can either change the counter by some value or do nothing
// at the 20th operation and every 40th operation after that the counters value is multiplied by the number of operations add added to a total
// the total is then returned
fn problem1(input: &String) -> String {
    let mut registerx = 1;
    let mut total = 0;
    let mut cycle = 1;

    // iterates through the input counting each one as a operation
    // operations that take additional time are handled withing the loop
    for line in input.lines() {
        
        // check at the start of a cycle if the counter is valid
        countWhenValid(&mut total, &cycle, &registerx);

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
            
            countWhenValid(&mut total, &cycle, &registerx);

            // gets the value to add to the counter
            // parses it to an i32 and unwraps it
            // adds it to the counter
            let value = split.next().unwrap().parse::<i32>().unwrap();
            registerx += value;

            cycle += 1; // TLDR: wait 1
            
        } else {
            // if the instruction is not noop then it is a noop
            // so we just add one to the counter
            cycle += 1;
        }

    }

    return total.to_string();
}

fn countWhenValid(total: &mut i32, cycle: &i32, registerx: &i32) {
    if (cycle + 20) % 40 == 0 {
        *total += registerx * cycle;
    }
}

// @dev builds an output array that mimics the way a crt works
// @param input a list of instructions that are either a noop or a addx
fn problem2(input: &String) -> Vec<String> {
    const SCREEN_WIDTH: i32 = 40;
    const START_CYCLE: i32 = 0;

    // the output array that will be built through the process
    let mut screenOutput = Vec::<String>::new();

    let mut registerx = 1;
    let mut cycle = START_CYCLE+1;

    let mut currentRow: String = String::new();

    // explian with this needs to exist for the code to function as intended
    renderPixel(&mut currentRow, &registerx, &cycle, &mut screenOutput, &SCREEN_WIDTH, &START_CYCLE);

    for line in input.lines() {
        // parse the operation into an instruction
        let mut split = line.split_whitespace();
        let instruction = split.next().unwrap();

        // parsing the instructions
        match instruction {
            "addx" => {
                renderPixel(&mut currentRow, &registerx, &cycle, &mut screenOutput, &SCREEN_WIDTH, &START_CYCLE);
                cycle += 1;
                

                let value = split.next().unwrap().parse::<i32>().unwrap();
                registerx += value;

                renderPixel(&mut currentRow, &registerx, &cycle, &mut screenOutput, &SCREEN_WIDTH, &START_CYCLE);
                cycle += 1;
                
            },
            "noop" => {
                renderPixel(&mut currentRow, &registerx, &cycle, &mut screenOutput, &SCREEN_WIDTH, &START_CYCLE);
                cycle += 1;
            },
            _ => {
                println!("Error: invalid instruction");
            }
        }
    }

    return screenOutput;
}


fn renderPixel(currentRow: &mut String, registerx: &i32, cycle: &i32, screenOutput: &mut Vec<String>, screen_width: &i32, start_cycle: &i32) {

    // the value of the register is the x position of the pixie
    // the pixie is 3 pixels wide and occupies the entire hieght of the screen
    // the cycle is what pixel is being rendered on the screen
    // each row has 'screen_width' pixels and so going over that number will wrap around to the next row

    // if the current row is full then add it to the output array
    // and reset the current row
    if cycle % screen_width == *start_cycle {
        screenOutput.push(currentRow.clone());
        currentRow.clear();
    }

    // if the pixie is within the current row then add "#" to the current row
    // if not then add "." to the current row
    if registerx-1 <= cycle%screen_width && cycle%screen_width <= *registerx+1 {
        currentRow.push_str("#");
    } else {
        currentRow.push_str(".");
    }

    
    
}
