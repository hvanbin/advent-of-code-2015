#[derive(Clone, Debug, PartialEq)]
pub(super) enum Operation {
    OFF = 0,
    ON = 1,
    TOGGLE = 2,
}

impl Operation {
    pub fn from(text: &str) -> Self {
        let opcode: Operation;
        if text.contains("off") {
            opcode = Operation::OFF;
        } else if text.contains("on") {
            opcode = Operation::ON;
        } else {
            opcode = Operation::TOGGLE;
        }
        opcode
    }
}
