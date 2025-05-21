use super::instruction::Instruction;
use super::operation::Operation;
use ndarray::prelude::*;

pub(super) struct Grid {
    lights: Array<i32, Ix2>,
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            lights: Array::<i32, Ix2>::zeros((1000, 1000).f()),
        }
    }

    /// From the instruction, get a view into the array.
    /// Then mutate that view.
    pub fn switch(&mut self, instruction: &Instruction) {
        let mut lights = self.lights.view_mut();
        let mut lights_of_interest =
            lights.slice_mut(s![instruction.get_y_range(), instruction.get_x_range()]);
        match instruction.get_operation() {
            Operation::OFF => lights_of_interest.fill(0),
            Operation::ON => lights_of_interest.fill(1),
            Operation::TOGGLE => {
                for light in lights_of_interest {
                    if *light == 1 {
                        *light = 0;
                    } else {
                        *light = 1;
                    }
                }
            }
        }
    }

    /// Function dial uses the instruction to broadcast a scalar to the grid.
    /// This is matrix terminology for applying a smaller array (in this case, a single scalar value) to the grid.
    pub fn dial(&mut self, instruction: &Instruction) {
        let mut lights = self.lights.view_mut();
        let lights_of_interest =
            lights.slice_mut(s![instruction.get_y_range(), instruction.get_x_range()]);
        match instruction.get_operation() {
            Operation::OFF => {
                azip!((light_of_interest in lights_of_interest) if *light_of_interest > 0 { *light_of_interest -= 1 })
            }
            Operation::ON => {
                azip!((light_of_interest in lights_of_interest) *light_of_interest += 1)
            }
            Operation::TOGGLE => {
                azip!((light_of_interest in lights_of_interest) *light_of_interest += 2)
            }
        }
    }
    pub fn count_lights(&self) -> i32 {
        self.lights.sum()
    }
}

#[cfg(test)]
pub mod test_grid {
    use super::*;
    #[test]
    pub fn test_apply() {
        let mut grid = Grid::new();
        let instructions = [
            Instruction::from("turn on 0,0 to 2,2"),
            Instruction::from("turn off 1,1 to 1,2"),
            Instruction::from("toggle 1,1 to 2,1"),
        ];
        for instruction in instructions {
            grid.switch(&instruction);
        }
        assert_eq!(
            grid.lights.view().slice(s![0..3, 0..3]),
            array![[1, 1, 1], [1, 1, 0], [1, 0, 1]]
        );
    }
    pub fn test_count_lights() {
        let mut grid = Grid::new();
        let instructions = [
            Instruction::from("turn on 0,0 to 2,2"),
            Instruction::from("turn off 1,1 to 1,2"),
            Instruction::from("toggle 1,1 to 2,1"),
        ];
        for instruction in instructions {
            grid.switch(&instruction);
        }
        assert_eq!(7, grid.count_lights());
    }
}
