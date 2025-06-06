/// Some of the Terminal instances are variables. 
/// This file defines a lookup table for those variable names and their values.
/// Type information is held in the Terminal itself.

use std::collections::HashMap;
use std::any::Any;

// Simple mapping from String => Value
#[derive(Debug, Default)]
pub struct VariableContext {
    variables: HashMap<String, Box<dyn Any>>,
}

impl VariableContext {
    pub fn new() -> Self {
        VariableContext {
            variables: HashMap::new(),
        }
    }

    // remember, type is enforced in the actual tree.
    pub fn add_variable(&mut self, name: String, value: Box<dyn Any>) {
        self.variables.insert(name.clone(), value);
    }

    pub fn get_variable(&self, name: &str) -> Option<&Box<dyn Any>> {
        self.variables.get(name)
    }

    pub fn set_variable_value(&mut self, name: &str, value: Box<dyn Any>) -> Result<(), String> {
        if let Some(variable) = self.variables.get_mut(name) {
            *variable = value;  // Dereference to assign new value
            Ok(())
        } else {
            Err(format!("Variable '{}' not found", name))
        }
    }
}


