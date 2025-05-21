use regex::Regex;
use std::ops::RangeInclusive;

use super::operation::Operation;

#[derive(Debug, PartialEq)]
pub(super) struct Instruction {
    opcode: Operation,
    origin: (i32, i32),
    end: (i32, i32),
}

impl Instruction {
    pub(super) fn from(text: &str) -> Self {
        let opcode = Operation::from(text);
        let number_pattern = Regex::new(r"\d+").unwrap();
        let numbers: Vec<i32> = number_pattern
            .find_iter(text)
            .filter_map(|matching| matching.as_str().parse().ok())
            .collect();
        let origin = (numbers[0], numbers[1]);
        let end = (numbers[2], numbers[3]);
        Instruction {
            opcode,
            origin,
            end,
        }
    }

    pub(super) fn get_x_range(&self) -> RangeInclusive<i32> {
        self.origin.1..=self.end.1
    }

    pub(super) fn get_y_range(&self) -> RangeInclusive<i32> {
        self.origin.0..=self.end.0
    }

    pub(super) fn get_operation(&self) -> Operation {
        self.opcode.clone()
    }
}

#[cfg(test)]
pub mod test_instruction {
    use super::*;
    #[test]
    pub fn test_instruction_from() {
        let input_strings = [
            "turn off 199,133 through 461,193",
            "toggle 322,558 through 977,958",
            "turn on 226,196 through 599,390",
        ];
        let expected_output_instruction = Instruction {
            opcode: Operation::OFF,
            origin: (199, 133),
            end: (461, 193),
        };
        assert_eq!(
            expected_output_instruction,
            Instruction::from(input_strings[0])
        );
        let expected_output_instruction = Instruction {
            opcode: Operation::TOGGLE,
            origin: (322, 558),
            end: (977, 958),
        };
        assert_eq!(
            expected_output_instruction,
            Instruction::from(input_strings[1])
        );
        let expected_output_instruction = Instruction {
            opcode: Operation::ON,
            origin: (226, 196),
            end: (599, 390),
        };
        assert_eq!(
            expected_output_instruction,
            Instruction::from(input_strings[2])
        );
    }
}
