//! This solves both part 1 and part 2 of day 2 of Advent of Code 2015.
use std::io::BufRead;

struct Present {
    length: i32,
    width: i32,
    height: i32,
}

impl Present {
    fn new(length: i32, width: i32, height: i32) -> Present {
        Present {
            length,
            width,
            height,
        }
    }

    fn from(text: &str) -> Present {
        let mut dimensions = text.split('x').filter_map(|s| s.parse::<i32>().ok());
        Present::new(
            dimensions.next().unwrap(),
            dimensions.next().unwrap(),
            dimensions.next().unwrap(),
        )
    }

    fn get_surface_area_sqft(&self) -> i32 {
        2 * self.length * self.width + 2 * self.width * self.height + 2 * self.length * self.height
    }

    /// Function get_shortest_sides calculates the shortest sides of the present.
    ///
    /// Returns:
    ///     a tuple of the shortest side and the second shortest side
    fn get_shortest_sides_ft(&self) -> (i32, i32) {
        let shortest_side_candidate_1 = std::cmp::min(self.length, self.width);
        let shortest_side_candidate_2 = std::cmp::min(self.length, self.height);
        let shortest_side_candidate_3 = std::cmp::min(self.width, self.height);
        let shortest_side;
        let second_shortest_side;

        if shortest_side_candidate_1 < shortest_side_candidate_2 {
            second_shortest_side = shortest_side_candidate_2;
            shortest_side = shortest_side_candidate_1;
        } else if shortest_side_candidate_1 > shortest_side_candidate_2 {
            second_shortest_side = shortest_side_candidate_1;
            shortest_side = shortest_side_candidate_2;
        } else {
            second_shortest_side = shortest_side_candidate_3;
            shortest_side = shortest_side_candidate_1;
        };
        (shortest_side, second_shortest_side)
    }

    /// Function get_smallest_area_sqft is used to calculate the "slack" needed for the wrapping.
    fn get_smallest_area_sqft(&self) -> i32 {
        let (shortest_side_ft, second_shortest_side_ft) = self.get_shortest_sides_ft();
        shortest_side_ft * second_shortest_side_ft
    }

    fn get_wrappping_paper_sqft(&self) -> i32 {
        self.get_surface_area_sqft() + self.get_smallest_area_sqft()
    }

    fn get_shortest_perimeter_ft(&self) -> i32 {
        let (shortest_side_ft, second_shortest_side_ft) = self.get_shortest_sides_ft();
        shortest_side_ft * 2 + second_shortest_side_ft * 2
    }

    fn get_volume(&self) -> i32 {
        self.length * self.width * self.height
    }

    fn get_total_ribbon(&self) -> i32 {
        self.get_shortest_perimeter_ft() + self.get_volume()
    }
}

pub fn solve() {
    println!("Day 2");
    println!("-----\n");
    let input_filename = String::from("data/real/input.2.txt");
    let input_file = std::fs::File::open(input_filename).unwrap();
    let input_buf_reader = std::io::BufReader::new(input_file);
    let mut total_wrapping_paper_sqft = 0;
    let mut total_ribbon_length_ft = 0;
    for input_buf in input_buf_reader.lines() {
        let line = input_buf.unwrap();
        let present = Present::from(&line);
        total_wrapping_paper_sqft += present.get_wrappping_paper_sqft();
        total_ribbon_length_ft += present.get_total_ribbon();
    }
    println!("Total wrapping paper area: {}", total_wrapping_paper_sqft);
    println!("Total ribbon length: {}", total_ribbon_length_ft);
    println!("\n");
}

#[cfg(test)]
pub mod test_day_two {
    use super::*;
    const LENGTH: i32 = 3;
    const WIDTH: i32 = 4;
    const HEIGHT: i32 = 5;
    const PRESENT: Present = Present {
        length: LENGTH,
        width: WIDTH,
        height: HEIGHT,
    };

    #[test]
    pub fn test_get_shortest_sides_ft() {
        assert_eq!(PRESENT.get_shortest_sides_ft(), (LENGTH, WIDTH));
    }

    #[test]
    pub fn test_get_surface_area_sqft() {
        assert_eq!(PRESENT.get_surface_area_sqft(), 94);
    }

    #[test]
    pub fn test_get_smallest_area_sqft() {
        assert_eq!(PRESENT.get_smallest_area_sqft(), 12);
    }

    #[test]
    pub fn test_get_wrapping_paper_sqft() {
        assert_eq!(PRESENT.get_wrappping_paper_sqft(), 106)
    }

    #[test]
    pub fn test_from() {
        let present = Present::from("3x4x5");
        assert_eq!(present.length, 3);
        assert_eq!(present.width, 4);
        assert_eq!(present.height, 5);
    }

    #[test]
    pub fn test_get_shortest_perimeter() {
        assert_eq!(PRESENT.get_shortest_perimeter_ft(), 14);
    }

    #[test]
    pub fn test_get_volume() {
        assert_eq!(PRESENT.get_volume(), 60);
    }

    #[test]
    pub fn test_get_total_ribbon() {
        assert_eq!(PRESENT.get_total_ribbon(), 74);
    }
}
