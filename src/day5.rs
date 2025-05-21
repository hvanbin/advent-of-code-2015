//! This solves day 5 parts 1 and 2 of Advent of Code 2015.
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek};

pub fn verify_niceness(candidate_string: &str) -> bool {
    let vowel_count_check = candidate_string.matches(['a', 'e', 'i', 'o', 'u']).count() >= 3;

    // This iterator is a "template" to be cloned by the matching_pair and no_illegal_pairs below.
    let lookahead_pair = candidate_string
        .chars()
        .zip(candidate_string.chars().skip(1));

    let matching_pair = lookahead_pair.clone().any(|(a, b)| a == b);

    let illegal_pairs = [('a', 'b'), ('c', 'd'), ('p', 'q'), ('x', 'y')];
    let no_illegal_pairs = lookahead_pair
        .clone()
        .all(|(a, b)| !illegal_pairs.contains(&(a, b)));
    vowel_count_check && matching_pair && no_illegal_pairs
}

pub fn part_one(buf_reader: &mut BufReader<File>) {
    let nice_count = buf_reader
        .by_ref()
        .lines()
        .filter_map(Result::ok)
        .filter(|x| verify_niceness(&x))
        .count();
    println! {"Part 1 (count of nice strings): {}", nice_count};
}

/// Check two conditions.
/// Iterate through substrings of two.
/// Check if there are any other substrings of two in the rest of the string that match.
/// Iterate through substrings of three.
/// Check for a pattern of xyx.
pub fn verify_niceness2(candidate_string: &str) -> bool {
    let pairs = candidate_string
        .chars()
        .zip(candidate_string.chars().skip(1));
    let mut nice2a = false;
    let enumerated_pairs = pairs.clone().enumerate();
    for (idx, pair) in enumerated_pairs {
        let mut other_pairs = pairs.clone().skip(idx + 2);
        nice2a |= other_pairs.any(|other_pair| pair == other_pair);
    }
    let mut spaced_pairs = candidate_string
        .chars()
        .zip(candidate_string.chars().skip(2));
    let nice2b = spaced_pairs.any(|(a, b)| a == b);
    nice2a && nice2b
}

pub fn part_two(buf_reader: &mut BufReader<File>) {
    println!("Reached part two");
    let nice_count = buf_reader
        .by_ref()
        .lines()
        .filter_map(Result::ok)
        .filter(|x| verify_niceness2(&x))
        .count();
    println! {"Part 2 (count of nice strings): {}\n", nice_count};
}

pub fn solve() {
    println!("Day 5\n-----\n");
    let in_filename = "data/real/input.5.txt";
    if let Ok(in_file) = File::open(in_filename) {
        let mut buf_reader = BufReader::new(in_file);
        part_one(&mut buf_reader);
        if let Ok(_) = buf_reader.rewind() {
            part_two(&mut buf_reader);
        } else {
            eprintln!("Failed to rewind the buffer for part two.");
        }
    } else {
        eprintln!("The input file {} could not be found!", in_filename);
    }
}

#[cfg(test)]
pub mod test_day5 {
    use super::*;
    #[test]
    pub fn test_verify_niceness() {
        let candidate_string = "ugknbfddgicrmopn";
        assert!(verify_niceness(candidate_string));
        let candidate_string = "aaa";
        assert!(verify_niceness(candidate_string));
        let candidate_string = "jchzalrnumimnmhp";
        assert!(!verify_niceness(candidate_string));
        let candidate_string = "haegwjzuvuyypxyu";
        assert!(!verify_niceness(candidate_string));
        let candidate_string = "dvszwmarrgswjxmb";
        assert!(!verify_niceness(candidate_string));
    }

    #[test]
    pub fn test_verify_niceness2() {
        assert!(verify_niceness2("qjhvhtzxzqqjkmpb"));
        assert!(verify_niceness2("xxyxx"));
        assert!(!verify_niceness2("uurcxstgmygtbstg"));
        assert!(!verify_niceness2("ieodomkazucvgmuy"))
    }
}
