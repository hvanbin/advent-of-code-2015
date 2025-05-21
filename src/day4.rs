use md5;

pub fn get_md5_input(arg_lowest_number: i32, arg_key: &str) -> Vec<u8> {
    let lowest_string = arg_lowest_number.to_string();
    let input_string = arg_key.to_string() + &lowest_string;
    input_string.into_bytes()
}

pub fn calculate(arg_key: &str, num_zeros: usize) -> i32 {
    let mut lowest_number = 1;
    let target_zeros = "0".repeat(num_zeros);
    loop {
        let md5_input = get_md5_input(lowest_number, arg_key);
        let output_hash = md5::compute(&md5_input);
        let output_string = format!("{:x}", output_hash);
        if &output_string[0..num_zeros] == target_zeros {
            break lowest_number;
        } else {
            lowest_number += 1;
        }
    }
}

/// Read the secret key from the input file.
/// Input it to the calculation function.
/// Print out the resulting number.
pub fn solve() {
    println!("Day 4\n-----\n");
    if let Ok(in_key) = std::fs::read_to_string("data/real/input.4.txt") {
        let num_zeros: usize = 5;
        let out_number = calculate(&in_key, num_zeros);
        println!("five zeros: {}", out_number);
        let num_zeros: usize = 6;
        let out_number = calculate(&in_key, num_zeros);
        println!("six zeros: {}\n", out_number);
    } else {
        println!("Could not calculate with input file.");
    }
}

#[cfg(test)]
pub mod test_day4 {
    use super::*;

    #[test]
    pub fn test_get_md5_input() {
        let in_key = "abcdef";
        let lowest_number = 609043;
        let md5_input = b"abcdef609043";
        assert_eq!(&get_md5_input(lowest_number, in_key), md5_input);
    }

    #[test]
    pub fn test_md5_output() {
        let md5_input = b"abcdef609043";
        let md5_output = md5::compute(md5_input);
        println!("{:?}", md5_output);
        let output_string = format!("{:x}", md5_output);
        let expected_md5_output_substring = "000001dbbfa";
        assert_eq!(&output_string[0..11], expected_md5_output_substring);
    }

    #[test]
    pub fn test_calculate() {
        let in_key = "abcdef";
        let num_zeros: usize = 5;
        let out_number = calculate(in_key, num_zeros);
        assert_eq!(out_number, 609043);
        let in_key = "pqrstuv";
        let out_number = calculate(in_key, num_zeros);
        assert_eq!(out_number, 1048970);
    }
}
