use regex::Regex;

pub(super) struct WireParser {
    re_binary: Regex,
    re_unary: Regex,
    re_assign: Regex
}

impl WireParser {

    // These are associated constants, which must exist in an `impl` in Rust.
    const PAT_BINARY: &str = r"(?<operand>[A-Za-z]+) (?<operator>AND|OR|LSHIFT|RSHIFT) (?<operand2>[0-9A-Za-z]+) -> (?<destination>[A-Za-z])";
    const PAT_UNARY: &str = r"(?<operator>[A-Z]+) (?<operand>[A-Za-z]+) -> (?<destination>[A-Za-z])";
    const PAT_ASSIGN: &str = r"(?<value>[0-9]+) -> (?<destination>[A-Za-z]+)";

    pub fn new() -> Self {
        WireParser {
            re_binary: Regex::new(WireParser::PAT_BINARY).unwrap(),
            re_unary: Regex::new(WireParser::PAT_UNARY).unwrap(),
            re_assign: Regex::new(WireParser::PAT_ASSIGN).unwrap(),
        }
    }

    pub fn parse(&self, in_line: &str) -> Option<WireSchematic> {
        dbg!(in_line);
        if let Some(caps) = self.re_binary.captures(&in_line) {
            
            let (_, [operand, operator, operand2, destination]) = caps.extract();
            Some(WireSchematic {
                destination: String::from(destination),
                operand: String::from(operand),
                operator: Some(String::from(operator)),
                operand2: Some(String::from(operand2)),
            })

        } else if let Some(caps) = self.re_unary.captures(&in_line) {
            let (_, [operator, operand, destination]) = caps.extract();
            Some(WireSchematic {
                destination: String::from(destination),
                operand: String::from(operand),
                operator: Some(String::from(operator)),
                operand2: None,
            })
            
        } else if let Some(caps) = self.re_assign.captures(&in_line) {
            let (_, [value, destination]) = caps.extract();
            Some(WireSchematic {
                destination: String::from(destination),
                operand: String::from(value),
                operator: None,
                operand2: None,
            })
        } else {
            eprintln!("No captures");
            None
        }
    }
}

/// TODO: Fill out the tests.
#[cfg(test)]
pub mod test_day7 {
    use super::*;
    #[test]
    pub fn test_closure_from_line() {
        let in_lines = vec![
            "123 -> x",
            "456 -> y",
            "x AND y -> d",
            "x OR y -> e",
            "x LSHIFT 2 -> f",
            "y RSHIFT 2 -> g",
            "NOT x -> h",
            "NOT y -> i",
        ];
        let wire_parser = WireParser::new();
        let mut iter_lines = in_lines.into_iter();
        let wire1 = wire_parser.parse(iter_lines.next().unwrap()).unwrap();
        let expected_wire1 = WireSchematic {
            destination: String::from("x"),
            operand: String::from("123"),
            operator: None,
            operand2: None,
        };
        assert_eq!(wire1, expected_wire1);

        //assert_eq!(iter_wires.next().unwrap(), (String::from("y"), String::from("456"), String::new(), String::new()));
        //assert_eq!(iter_wires.next().unwrap(), (String::from("d"), String::from("AND"), String::from("x"), String::from("y")));
        //assert_eq!(iter_wires.next().unwrap(), (String::from("e"), String::from("OR"), String::from("x"), String::from("y")));
        //assert_eq!(iter_wires.next().unwrap(), (String::from("f"), String::from("LSHIFT"), String::from("x"), String::from("2")));
        //assert_eq!(iter_wires.next().unwrap(), (String::from("g"), String::from("RSHIFT"), String::from("y"), String::from("2")));
        //assert_eq!(iter_wires.next().unwrap(), (String::from("h"), String::from("NOT"), String::from("x"), String::new()));
        //assert_eq!(iter_wires.next().unwrap(), (String::from("i"), String::from("NOT"), String::from("y"), String::new()));
        //assert_eq!(iter_wires.next(), None);
    }
}