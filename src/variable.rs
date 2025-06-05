/// Some of the Terminal instances are variables. 
/// This file defines a lookup table for those variable names and their values.
/// Type information is held in the Terminal itself.

use std::collections::HashMap;
use std::any::Any;
use crate::types::{DataType, Shape};

#[derive(Debug)]
pub struct Variable {
    pub name: String,
    pub data_type: DataType,
    pub shape: Shape,
    pub value: Box<dyn Any>,
}

impl Variable {
    pub fn new<T: 'static>(name: String, data_type: DataType, shape: Shape, value: T) -> Self {
        Variable {
            name,
            data_type,
            shape,
            value: Box::new(value),
        }
    }
}

#[derive(Debug, Default)]
pub struct VariableContext {
    variables: HashMap<String, Variable>,
}

impl VariableContext {
    pub fn new() -> Self {
        VariableContext {
            variables: HashMap::new(),
        }
    }

    pub fn add_variable(&mut self, variable: Variable) {
        self.variables.insert(variable.name.clone(), variable);
    }

    pub fn get_variable(&self, name: &str) -> Option<&Variable> {
        self.variables.get(name)
    }

    pub fn set_variable_value<T: 'static>(&mut self, name: &str, value: T) -> Result<(), String> {
        if let Some(variable) = self.variables.get_mut(name) {
            variable.value = Box::new(value);
            Ok(())
        } else {
            Err(format!("Variable '{}' not found", name))
        }
    }

    pub fn get_variable_value(&self, name: &str) -> Option<&Box<dyn Any>> {
        self.variables.get(name).map(|var| &var.value)
    }

    pub fn get_variable_type(&self, name: &str) -> Option<(DataType, Shape)> {
        self.variables.get(name).map(|var| (var.data_type, var.shape))
    }
}


