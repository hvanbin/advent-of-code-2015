//! This is a solution for day 7 of advent of code.
//! https://adventofcode.com/2015/day/7
//! This program takes the input from `input.txt` and outputs to stdout.
//! The solution is essentially a collection of named functions generated at runtime.
//! These functions can call one another.

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Parse the line for the name, and either operator and operands, or value.
//pub fn closure_from_line(circuitry: &HashMap<String, Box<dyn Fn() -> i32>>, in_line: String) -> (String, Box<dyn Fn() -> i32>) {
pub fn closure_from_line(circuitry: &mut HashMap<String, Box<dyn Fn() -> i32>>, in_line: String) {
    // Parse for the name.
    // Parse for the operator or value.
    // If an operator, parse for the operands.
    // If operands, try to evaluate them now.
    // ...
    // Rebuild the closure.
    //circuitry.insert(String::from("a"), Box::new(|| 5));
    println!("Saw in_line: {}", in_line);
}

/// Function solve solves both parts one and two.
pub fn solve() {

    // Map "circuitry" maps the name of a wire to the expression that generates it.
    // Each expression is either an integer or a binary operation.
    // These are read in from `input.txt` at runtime.
    let mut circuitry: HashMap<String, Box<dyn Fn() -> i32>> = HashMap::new();

    // Open the input file.
    const IN_FILENAME: &str = "data/real/input.7.txt";
    if let Ok(in_file) = File::open(IN_FILENAME) {
        let buf_reader = BufReader::new(in_file);
        let in_lines = buf_reader.lines().filter_map(Result::ok);
        // Read each line.
        for in_line in in_lines {
            closure_from_line(&mut circuitry, in_line);
        }
    }

    // Assign the map at that string the appropriate closure.
    //   If a value, simply return the value in the closure.
    //   If an operator and operands, check if the operands can already be resolved.
    //   If so, resolve one or both of them.
    //   If both are resolved, evaluate the operator and simply assign the result to the closure.
    //   If only one, return a simplified closure with the result and the operator and remaining operand.

    // After the map is built, evaluate circuitry["a"]();
    // TODO: Better yet, print all signals on all wires as shown in the example.
    //dbg!(circuitry["a"]());
}

/// TODO: Fill out the tests.
#[cfg(test)]
pub mod test_day7 {
    use super::*;
    #[test]
    pub fn test_closure_from_line() {

    }
}