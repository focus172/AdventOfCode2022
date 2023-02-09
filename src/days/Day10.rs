use std::path;
use std::fs::File;
use std::io::{BufReader, BufRead, Error};

pub fn main() {
    //https://textkool.com/en/test-ascii-art-generator


    // prints block text reading "Day 10"
    println!("
██████╗  █████╗ ██╗   ██╗     ██╗ ██████╗ 
██╔══██╗██╔══██╗╚██╗ ██╔╝    ███║██╔═████╗
██║  ██║███████║ ╚████╔╝     ╚██║██║██╔██║
██║  ██║██╔══██║  ╚██╔╝       ██║████╔╝██║
██████╔╝██║  ██║   ██║        ██║╚██████╔╝
╚═════╝ ╚═╝  ╚═╝   ╚═╝        ╚═╝ ╚═════╝ 
    ");


    // gets the input from the file in the form of a vector of strings
    let wrapped_input = read_from_file_to_lines("./src/inputs/Day10Input.txt");

    // unwraps the input and prints an error if it fails
    let input = match wrapped_input {
        Ok(input) => input,
        Err(error) => {
            println!("Error: {}", error);
            return;
        }
    };

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
}

// this code is responsible for keeping track on one counter as it runs through the input
// each input can either change the counter by some value or do nothing
// at the 20th operation and every 40th operation after that the counters value is multiplied by the number of operations add added to a total
// the total is then returned
fn problem1(input: &Vec<String>) -> String {
    let mut registerx = 1;
    let mut total = 0;
    let mut cycle = 1;

    // iterates through the input counting each one as a operation
    // operations that take additional time are handled withing the loop
    for operation in input {
        
        // check at the start of a cycle if the counter is valid
        countWhenValid(&mut total, &cycle, &registerx);

        // parse the operation into an instruction
        let mut split = operation.split_whitespace();
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
fn problem2(input: &Vec<String>) -> Vec<String> {
    const SCREEN_WIDTH: i32 = 40;

    // the output array that will be built through the process
    let mut screenOutput = Vec::<String>::new();

    let mut registerx = 1;
    let mut cycle = 1;

    let mut currentRow: String = String::new();

    for operation in input {
        // parse the operation into an instruction
        let mut split = operation.split_whitespace();
        let instruction = split.next().unwrap();

        // parsing the instructions
        match instruction {
            "addx" => {
                cycle += 1;
                renderPixel(&mut currentRow, &registerx, &cycle, &mut screenOutput, &SCREEN_WIDTH);

                let value = split.next().unwrap().parse::<i32>().unwrap();
                registerx += value;

                cycle += 1;
                renderPixel(&mut currentRow, &registerx, &cycle, &mut screenOutput, &SCREEN_WIDTH);
            },
            "noop" => {
                cycle += 1;

                renderPixel(&mut currentRow, &registerx, &cycle, &mut screenOutput, &SCREEN_WIDTH);
            },
            _ => {
                println!("Error: invalid instruction");
            }
        }
    }

    return screenOutput;
}


fn renderPixel(currentRow: &mut String, registerx: &i32, cycle: &i32, screenOutput: &mut Vec<String>, screen_width: &i32) {

    // the value of the register is the x position of the pixie
    // the pixie is 3 pixels wide and occupies the entire hieght of the screen
    // the cycle is what pixel is being rendered on the screen
    // each row has 'screen_width' pixels and so going over that number will wrap around to the next row

    // if the pixie is within the current row then add "#" to the current row
    // if not then add "." to the current row
    if registerx-1 <= cycle%screen_width && cycle%screen_width <= *registerx+1 {
        currentRow.push_str("#");
    } else {
        currentRow.push_str(".");
    }

    // if the current row is full then add it to the output array
    // and reset the current row
    if cycle % screen_width == 1 {
        screenOutput.push(currentRow.clone());
        currentRow.clear();
    }
    
}

fn read_from_file_to_lines(filename: &str) -> Result<Vec<String>, Error> {
    // gets the pat of the file
    let path = path::Path::new(filename);

    println!("Reading from file: {}...", filename);

    // get the file from the path 
    let wrappped_file = File::open(&path);
    let file = match wrappped_file {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    // create a buffered reader to read the file
    let reader = BufReader::new(file);

    // iterate through the lines of the file and add them to a vector
    let mut lines: Vec<String> = Vec::new();
    reader.lines().for_each(|line| {
        lines.push(line.unwrap());
    });

    // return the vector
    return Ok(lines)
}
