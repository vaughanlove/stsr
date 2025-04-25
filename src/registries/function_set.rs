use crate::types::terminal::{GenericFunction, TerminalShape};
use std::collections::HashMap;

pub struct FunctionSet {
    accepted_functions: Vec<GenericFunction>,
    shape_lookup_table: HashMap<TerminalShape, Vec<usize>>,
}

impl FunctionSet {
    fn new() -> Self {
        FunctionSet {
            accepted_functions: Vec::new(),
            shape_lookup_table: HashMap::new(),
        }
    }

    fn add_function(&mut self, function: GenericFunction) {
        let function_idx = self.accepted_functions.len();
        let output = function.output.clone();

        self.accepted_functions.push(function);
        self.shape_lookup_table.entry(output).or_insert_with(Vec::new).push(function_idx)
    }
}
