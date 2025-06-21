/// Base Types in the system. 
/// 
/// TODO: The developer needs a way to specify a subset of these for their genetic program. 
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum DataType {
    Integer,
    Float,
}

/// The shape that a terminal can take. 
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Shape {
    Scalar,
    Vector(usize),
    Matrix(usize, usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
use std::rc::Rc;

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

#[derive(Debug)]
pub struct DataRow {
    pub values: HashMap<String, Rc<dyn Any>>,
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
            row_values.insert(var.name.clone(), Rc::from(values.remove(0)));
        }
        
        Ok(DataRow { values: row_values })
    }
    
    // Alternative constructor with explicit key-value pairs (with validation)
    pub fn from_map(variable_defs: &VariableDefinitions, values: HashMap<String, Box<dyn Any>>) -> Result<Self, String> {
        let rc_values: HashMap<String, Rc<dyn Any>> = values
            .into_iter()
            .map(|(k, v)| (k, Rc::from(v)))
            .collect();
        let row = DataRow { values: rc_values };
        variable_defs.validate_data_row(&row)?;
        Ok(row)
    }
}
#[derive(Debug)]
pub enum EvalInput<'a> {
    Data(&'a DataRow, &'a Rc<dyn Any>)
}

// Dataset containing input rows and expected outputs
#[derive(Debug)]
pub struct Dataset {
    pub features: Vec<DataRow>,
    pub targets: Vec<Rc<dyn Any>>,
}

impl Dataset {
    pub fn new(features: Vec<DataRow>, targets: Vec<Box<dyn Any>>) -> Result<Self, String> {
        if features.len() != targets.len() {
            return Err(format!(
                "Number of features ({}) must match number of targets ({})",
                features.len(),
                targets.len()
            ));
        }
        
        let rc_targets: Vec<Rc<dyn Any>> = targets.into_iter().map(Rc::from).collect();
        Ok(Dataset { features, targets: rc_targets })
    }

    pub fn sample_row(&self, index: usize) -> EvalInput {
        EvalInput::Data(&self.features[index], &self.targets[index])
    }
}