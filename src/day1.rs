//! This solves both part 1 and part 2 of day 1 of Advent of Code 2015.
use std::fs;

/// Parse the input string of directions to calculate the answers.
/// Answers include the count the number of floors Santa goes up and down, as well as when Santa reaches the basement floor.
///
/// Args:
///     inputs: Input string of parentheses.
///
/// Returns:
///     None
///
/// Example:
/// ```
/// use day1::day_one::parse_directions;
/// let directions = String::from("()())");
/// let (floor_level, basement_position) = parse_directions(directions);
/// assert_eq!(floor_level, -1);
/// assert_eq!(basement_position, 5);
/// ```
fn parse_directions(arg_directions: &str) -> (i32, usize) {
    let mut floor_level = 0;
    let mut basement_found = false;
    let mut basement_position = 0;
    for (index, direction) in arg_directions.char_indices() {
        if direction == '(' {
            floor_level += 1;
        } else if direction == ')' {
            floor_level -= 1;
        }

        if basement_found == false && floor_level == -1 {
            basement_found = true;
            basement_position = index + 1;
        }
    }
    (floor_level, basement_position)
}

pub fn solve() {
    println!("Day 1");
    println!("-----\n");
    let filename = "data/real/input.1.txt";
    let f_directions = fs::read_to_string(filename).expect("Error reading the input file");
    let (floor_level, basement_position) = parse_directions(&f_directions);
    println!(
        "floor level: {}\nbasement position: {}",
        floor_level, basement_position
    );
    println!("\n");
}

#[cfg(test)]
pub mod test_day1 {
    use super::*;
    #[test]
    pub fn test_parse_directions() {
        let directions = "()())";
        let expected_floor_level = -1;
        let expected_basement_position = 5;
        assert_eq!(
            parse_directions(directions),
            (expected_floor_level, expected_basement_position)
        );
    }
}
