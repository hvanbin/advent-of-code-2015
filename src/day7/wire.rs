pub(super) struct Wire<'a> {
    name: String,
    operand: Option<&'a Wire<'a>>,
    operator: Option<Box<dyn Fn() -> i32>>,
    operand2: Option<&'a Wire<'a>>,
    value: Option<i32>,
}

impl<'a> Wire<'a> {

    pub fn from(circuitry: &HashMap<String, Wire>, wire_schematic: WireSchematic) -> Self {
        let operand = circuitry.get(wire_schematic.get_operand());
        let operand2 = circuitry.get(wire_schematic.get_operand2().unwrap().as_str());
        /*let operator = match wire_schematic.get_operator() {
            Some(operator) if operator == "NOT" => {
                if operand.unwrap().is_resolvable() {
                    Some(Box::new(||!!operand.unwrap().eval()))
                }
            },
            None => None
        }
        let operator = match (operand, operand2) {
            (Some(operand), Some(operand2)) if operand.is_resolvable() && operand2.is_resolvable() => {

            }
        }*/
        Wire {
            name: wire_schematic.get_destination().to_string(),
            operand: operand,
            operator: None,
            operand2: operand2,
        }
    }

    pub fn is_resolvable(&self) -> bool {
        match (self.operand, self.operand2) {
            (Some(operand1), Some(operand2)) => operand1.is_resolvable() && operand2.is_resolvable(),
            _ => false
        }
    }

    pub fn resolve(&mut self) -> Option<i32> {
        if self.value != None { return self.value; }
        else if self.is_resolvable() {
            self.value = self.operator(self.operand.unwrap().operator(), self.operand2.unwrap().operator())

        }
    }
}