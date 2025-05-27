/// Struct Wire concisely holds all information that could be encoded from the input.
/// It represents a wire in one of these circuit diagrams.
#[derive(Debug, PartialEq)]
pub(super) struct WireSchematic {
    destination: String,
    operand: String,
    operator: Option<String>,
    operand2: Option<String>,
}

impl WireSchematic {
    pub fn get_destination(&self) -> &str {
        &self.destination
    }
    pub fn get_operand(&self) -> &str {
        &self.operand
    }
    pub fn get_operand2(&self) -> &Option<String> {
        &self.operand2
    }
    pub fn get_operator(&self) -> &Option<String> {
        &self.operator
    }
}