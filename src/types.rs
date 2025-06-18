/// Base Types in the system. 
/// 
/// TODO: The developer needs a way to specify a subset of these for their genetic program. 
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DataType {
    Integer,
    Float,
}

/// The shape that a terminal can take. 
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Shape {
    Scalar,
    Vector(usize),
    Matrix(usize, usize),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TypeInfo {
    pub shape: Shape,
    pub data_type: DataType
}

// Struct that will be public facing for developers to define their own variables according to their datasets.
#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub _type: TypeInfo
}

use std::collections::HashMap;
use std::any::Any;

// Variable definitions with explicit ordering and validation
#[derive(Debug, Clone)]
pub struct VariableDefinitions {
    pub variables: Vec<Variable>,
}

impl VariableDefinitions {
    pub fn new(variables: Vec<Variable>) -> Self {
        VariableDefinitions { variables }
    }
    
    pub fn get_variable_names(&self) -> Vec<&str> {
        self.variables.iter().map(|v| v.name.as_str()).collect()
    }
    
    pub fn validate_data_row(&self, row: &DataRow) -> Result<(), String> {
        // Check that row has exactly the required variables
        for var in &self.variables {
            if !row.values.contains_key(&var.name) {
                return Err(format!("Missing variable '{}' in data row", var.name));
            }
        }
        
        // Check for extra variables
        for key in row.values.keys() {
            if !self.variables.iter().any(|v| &v.name == key) {
                return Err(format!("Unexpected variable '{}' in data row", key));
            }
        }
        
        Ok(())
    }
}

/// In GPSR, there are two generation methods outlined.
/// Grow - Terminals and Nonterminals can appear at any depth - randomly chosen during construction. Leaves are always Terminals.
/// Full - The entire tree is filled up until max_depth - 1 with NonTerminals. The leaves are then all populated with Terminals. 
#[derive(Clone, Copy, Debug)]
pub enum GenerationMethod {
    Full,
    Grow,
}

// Strongly typed data row that must match variable definitions
#[derive(Debug)]
pub struct DataRow {
    pub values: HashMap<String, Box<dyn Any>>,
}

impl DataRow {
    // Constructor that enforces variable definitions (positional)
    pub fn new(variable_defs: &VariableDefinitions, mut values: Vec<Box<dyn Any>>) -> Result<Self, String> {
        if values.len() != variable_defs.variables.len() {
            return Err(format!(
                "Expected {} values, got {}", 
                variable_defs.variables.len(), 
                values.len()
            ));
        }
        
        let mut row_values = HashMap::new();
        for (_, var) in variable_defs.variables.iter().enumerate() {
            row_values.insert(var.name.clone(), values.remove(0));
        }
        
        Ok(DataRow { values: row_values })
    }
    
    // Alternative constructor with explicit key-value pairs (with validation)
    pub fn from_map(variable_defs: &VariableDefinitions, values: HashMap<String, Box<dyn Any>>) -> Result<Self, String> {
        let row = DataRow { values };
        variable_defs.validate_data_row(&row)?;
        Ok(row)
    }
}

// Dataset containing input rows and expected outputs
#[derive(Debug)]
pub struct Dataset {
    pub inputs: Vec<DataRow>,
    pub expected_outputs: Vec<Box<dyn Any>>,
}

impl Dataset {
    pub fn new(inputs: Vec<DataRow>, expected_outputs: Vec<Box<dyn Any>>) -> Result<Self, String> {
        if inputs.len() != expected_outputs.len() {
            return Err(format!(
                "Number of input rows ({}) must match number of expected outputs ({})",
                inputs.len(),
                expected_outputs.len()
            ));
        }
        
        Ok(Dataset { inputs, expected_outputs })
    }
}