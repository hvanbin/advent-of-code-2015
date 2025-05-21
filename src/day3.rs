//! This solves both part 1 and part 2 of day 3 of Advent of Code 2015.
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

trait Markable {
    fn mark(&mut self, coord: (i32, i32));
    fn get_marks(&self) -> i32;
}

/// This is an implementation of a grid which uses a HashMap.
struct Grid {
    map: HashMap<(i32, i32), bool>,
    count: i32,
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            map: HashMap::new(),
            count: 0,
        }
    }
}
impl Markable for Grid {
    fn mark(&mut self, coord: (i32, i32)) {
        if !self.map.contains_key(&coord) {
            self.map.insert(coord, true);
            self.count += 1;
        }
    }

    fn get_marks(&self) -> i32 {
        self.count
    }
}

// A version of Giver that uses Rc<RefCell<Grid>> directly
struct Giver {
    coord: (i32, i32),
    grid: std::rc::Rc<std::cell::RefCell<Grid>>,
}

impl Giver {
    pub fn new(grid: std::rc::Rc<std::cell::RefCell<Grid>>) -> Self {
        let origin = (0, 0);
        grid.borrow_mut().mark(origin);
        Giver {
            coord: origin,
            grid: grid,
        }
    }
    pub fn travel(&mut self, direction: char) {
        let (x, y) = self.coord;
        self.coord = match direction {
            '^' => (x, y + 1),
            '>' => (x + 1, y),
            'v' => (x, y - 1),
            '<' => (x - 1, y),
            _ => self.coord,
        };
        self.grid.borrow_mut().mark(self.coord);
    }
}

struct Director<'b> {
    directions: &'b str,
    givers: Vec<&'b mut Giver>,
}

impl<'b> Director<'b> {
    pub fn new(directions: &'b str, givers: Vec<&'b mut Giver>) -> Self {
        Director { directions, givers }
    }

    /// Go through the list of givers and send them instructions.
    pub fn direct(&mut self) {
        let mut giver_index = 0;
        for direction in self.directions.chars() {
            self.givers[giver_index].travel(direction);
            if giver_index < self.givers.len() - 1 {
                giver_index += 1;
            } else {
                giver_index = 0;
            }
        }
    }
}

fn part_one(in_directions: &str) -> i32 {
    // Create the shared grid
    let grid_rc = Rc::new(RefCell::new(Grid::new()));
    let mut giver = Giver::new(grid_rc.clone());
    let givers = vec![&mut giver];
    let mut director = Director::new(&in_directions, givers);
    director.direct();

    let borrowed_grid = grid_rc.borrow();
    borrowed_grid.get_marks()
}

fn part_two(in_directions: &str) -> i32 {
    // Create the shared grid
    let grid_rc = Rc::new(RefCell::new(Grid::new()));
    let mut giver1 = Giver::new(grid_rc.clone());
    let mut giver2 = Giver::new(grid_rc.clone());
    let givers = vec![&mut giver1, &mut giver2];
    let mut director = Director::new(&in_directions, givers);
    director.direct();

    let borrowed_grid = grid_rc.borrow();
    borrowed_grid.get_marks()
}

pub fn solve() {
    println!("Day 3\n-----\n");
    let in_filename = String::from("data/real/input.3.txt");
    if let Ok(in_string) = std::fs::read_to_string(in_filename) {
        let answer1 = part_one(&in_string);
        let answer2 = part_two(&in_string);
        println! {"Part 1: {}\nPart 2: {}\n", answer1, answer2};
    }
}

#[cfg(test)]
pub mod test_day_three {
    use super::*;

    #[test]
    pub fn test_grid_new() {
        let grid = Grid::new();
        assert_eq!(grid.count, 0);
    }

    #[test]
    pub fn test_grid_mark() {
        let mut grid = Grid::new();
        grid.mark((2, 3));
        assert_eq!(grid.map[&(2, 3)], true);
        assert!(!grid.map.contains_key(&(1, 2)));
    }

    #[test]
    pub fn test_grid_get_marks() {
        let mut grid = Grid::new();
        grid.mark((3, 4));
        grid.mark((4, 4));
        grid.mark((3, 4));
        assert_eq!(grid.get_marks(), 2);
    }

    #[test]
    pub fn test_giver_new() {
        let grid_rc = Rc::new(RefCell::new(Grid::new()));
        let _giver = Giver::new(grid_rc.clone());
        let borrowed_grid_rc = grid_rc.borrow();
        assert_eq!(borrowed_grid_rc.map[&(0, 0)], true);
        assert_eq!(borrowed_grid_rc.get_marks(), 1);
    }

    #[test]
    pub fn test_giver_travel() {
        let grid_rc = Rc::new(RefCell::new(Grid::new()));
        let mut giver = Giver::new(grid_rc.clone());
        giver.travel('^');
        giver.travel('>');
        giver.travel('v');
        giver.travel('<');
        assert_eq!(grid_rc.borrow().map[&(0, 0)], true);
        assert_eq!(grid_rc.borrow().map[&(0, 1)], true);
        assert_eq!(grid_rc.borrow().map[&(1, 1)], true);
        assert_eq!(grid_rc.borrow().map[&(1, 0)], true);
        assert_eq!(grid_rc.borrow().get_marks(), 4);
    }

    #[test]
    pub fn test_two_giver_travel() {
        // Create the shared grid
        let grid_rc = Rc::new(RefCell::new(Grid::new()));

        // Create two givers with the same shared grid
        let mut giver1 = Giver::new(grid_rc.clone());
        let mut giver2 = Giver::new(grid_rc.clone());

        // Have them travel
        giver1.travel('^');
        giver2.travel('>');
        giver1.travel('v');
        giver2.travel('<');

        // Check the results
        let borrowed_grid = grid_rc.borrow();
        assert_eq!(borrowed_grid.map[&(0, 0)], true);
        assert_eq!(borrowed_grid.map[&(0, 1)], true);
        assert_eq!(borrowed_grid.map[&(1, 0)], true);
        assert_eq!(borrowed_grid.get_marks(), 3);
    }

    #[test]
    pub fn test_direct() {
        // Create the shared grid
        let grid_rc = Rc::new(RefCell::new(Grid::new()));
        // Create two givers with the same shared grid
        let mut giver1 = Giver::new(grid_rc.clone());
        let mut giver2 = Giver::new(grid_rc.clone());
        let givers = vec![&mut giver1, &mut giver2];
        let directions = String::from("^>v<");
        let mut director = Director::new(&directions, givers);
        director.direct();

        let borrowed_grid = grid_rc.borrow();
        assert_eq!(borrowed_grid.map[&(0, 0)], true);
        assert_eq!(borrowed_grid.map[&(0, 1)], true);
        assert_eq!(borrowed_grid.map[&(1, 0)], true);
        assert_eq!(borrowed_grid.get_marks(), 3);
    }

    #[test]
    pub fn test_part_one() {
        let input = "^>v<";
        let output = 4;
        assert_eq!(part_one(input), output);
    }

    #[test]
    pub fn test_part_two() {
        let input = "^>v<";
        let output = 3;
        assert_eq!(part_two(input), output);
    }

    #[test]
    pub fn test_case_1() {}
}
