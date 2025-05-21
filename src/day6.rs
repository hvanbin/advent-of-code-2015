/// There are three ways that come to mind to solve this problem.
/// 1. The naive way is to create a map and apply every transformation.
/// 2. Create a stack of coordinate-ordered, non-overlaying grids. Write an algorithm to resolve collisions into subgrids.
/// 3. Another way is to simply use matrix operations (e.g. from a library).
///
/// Matrix operations are used here.
mod grid;
mod instruction;
mod operation;

use std::io::BufRead;

use grid::Grid;
use instruction::Instruction;

/// Function part_one uses the instructions to mutate the grid.
fn part_one<'a, I>(instructions: I) -> i32
where
    I: IntoIterator<Item = &'a Instruction>,
{
    let mut grid = Grid::new();
    for instruction in instructions {
        grid.switch(instruction)
    }
    grid.count_lights()
}

/// Function part_two uses the instructions to mutate the grid with the second algorithm.
fn part_two<'a, I>(instructions: I) -> i32
where
    I: IntoIterator<Item = &'a Instruction>,
{
    let mut grid = Grid::new();
    for instruction in instructions {
        grid.dial(instruction)
    }
    grid.count_lights()
}

pub fn solve() {
    println!("Day 6\n-----\n");
    let in_filename = "data/real/input.6.txt";
    if let Ok(in_file) = std::fs::File::open(in_filename) {
        let buf_reader = std::io::BufReader::new(in_file);
        let instructions: Vec<Instruction> = buf_reader
            .lines()
            .map(|line| Instruction::from(&line.unwrap()))
            .collect();
        let answer1 = part_one(&instructions);
        println!("Part 1: {}", answer1);
        let answer2 = part_two(&instructions);
        println!("Part 2: {}", answer2);
    } else {
        eprintln!("Could not open input 6!");
    }
}
