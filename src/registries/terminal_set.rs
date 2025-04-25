use crate::types;
use types::terminal::*;
use std::{collections::HashMap, vec::Vec};

pub struct TerminalSet {
    accepted_terminals: Vec<Terminal>,
    shape_lookup_table: HashMap<TerminalShape, Vec<usize>>
}

impl TerminalSet {
    pub fn new() -> Self  {
        Self  {
            accepted_terminals: Vec::new(),
            shape_lookup_table: HashMap::new()
        }
    }

    /// Add a terminal to the set of accepted terminals.
    pub fn add_terminal(&mut self, terminal: Terminal) {
        let shape = match &terminal {
            Terminal::Variable(shape) => shape.clone(),
            Terminal::Constant(shape) => shape.clone(),
        };

        let terminal_idx = self.accepted_terminals.len();
        self.accepted_terminals.push(terminal);
        
        self.shape_lookup_table
            .entry(shape)
            .or_insert_with(Vec::new)
            .push(terminal_idx);
    }

    /// Check if there are any terminals with a given shape
    pub fn has_terminals_with_shape(&self, shape: &TerminalShape) -> bool {
        self.shape_lookup_table.contains_key(shape) && !self.shape_lookup_table[shape].is_empty()
    }

    
}