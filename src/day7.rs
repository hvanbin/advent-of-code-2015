//! This is a solution for day 7 of advent of code.
//! https://adventofcode.com/2015/day/7
//! This program takes the input from `input.txt` and outputs to stdout.
//! The solution is essentially a collection of named functions generated at runtime.
//! These functions can call one another.

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

// Declare the submodules
mod wire;
mod wire_parser;
mod wire_schematic;

// Import the types we need
use wire::Wire;
use wire_parser::WireParser;
use wire_schematic::WireSchematic;

struct Circuit {
    circuitry: HashMap<&str, Box<dyn Fn() -> i32>>
}

impl Circuit {
    /// Assign the map at that string the appropriate closure.
    pub fn wire(&mut self, wire: WireSchematic) {
        if wire.operand == ""
        self.circuitry.insert(wire.get_destination(), Box::new(|| 5));
    }
}
// TODO: Holy cow, test and document this.
pub fn parse_line<'a>(regex_binary: &Regex, regex_unary: &Regex, re_assign: &Regex, in_lines: impl Iterator<Item = &'a str>) -> impl Iterator<Item = (String, String, String, String)> {
    let mut return_vector = vec![];
    // Parse for the name.
    for in_line in in_lines {
    }
    return_vector.into_iter()
}

// TODO: Change this to be closure_from_tuple.
/// Parse the line for the name, and either operator and operands, or value.
//pub fn closure_from_line(circuitry: &HashMap<String, Box<dyn Fn() -> i32>>, in_line: String) -> (String, Box<dyn Fn() -> i32>) {
pub fn closure_from_line(circuitry: &mut HashMap<String, Box<dyn Fn() -> i32>>, in_line: String) {
    // Parse for the operator or value.
    // If an operator, parse for the operands.
    // If operands, try to evaluate them now.
    // ...
    // Rebuild the closure.
    //circuitry.insert(String::from("a"), Box::new(|| 5));
}

/// Function solve solves both parts one and two.
pub fn solve() {

    // Map "circuitry" maps the name of a wire to the expression that generates it.
    // Each expression is either an integer or a binary operation.
    // These are read in from `input.txt` at runtime.
    let mut circuitry: HashMap<String, Wire> = HashMap::new();
    let wire_parser = WireParser::new();

    // Open the input file.
    const IN_FILENAME: &str = "data/real/input.7.txt";
    if let Ok(in_file) = File::open(IN_FILENAME) {
        let buf_reader = BufReader::new(in_file);
        let in_lines = buf_reader.lines().filter_map(Result::ok);
        for in_line in in_lines {
            let wire = wire_parser.parse(&in_line);
        }
    }

    //   If a value, simply return the value in the closure.
    //   If an operator and operands, check if the operands can already be resolved.
    //   If so, resolve one or both of them.
    //   If both are resolved, evaluate the operator and simply assign the result to the closure.
    //   If only one, return a simplified closure with the result and the operator and remaining operand.

    // After the map is built, evaluate circuitry["a"]();
    // TODO: Better yet, print all signals on all wires as shown in the example.
    //dbg!(circuitry["a"]());
}
