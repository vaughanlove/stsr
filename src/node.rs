use crate::ops::Operation;
use crate::types::{DataType, Shape};
use crate::registry::TypeRegistry;
use crate::variable::VariableContext;
use std::any::Any;

#[derive(Clone, Copy, Debug)]
pub enum NodeType {
    // Heap-allocated NonTerminal
    // input type, input type, operation, output type
    NonTerminal(Operation),

    // Heap-allocated Terminal
    Terminal(DataType, Shape),
}

#[derive(Debug)]
pub struct Node {
    pub idx: usize,
    pub variable_id: Option<String>, // for generics that need to pull value from variable.rs HashMap.
    pub _type: NodeType, // for GPSR
    pub value: Box<dyn Any>,
    pub left: Option<usize>,
    pub right: Option<usize>,
    pub parent: usize,
}

pub trait MatchesTerminal {
    const DATA_TYPE: DataType;
    fn get_shape(&self) -> Shape; 
}

impl MatchesTerminal for i64 {
    const DATA_TYPE: DataType = DataType::Integer;
    fn get_shape(&self) -> Shape { Shape::Scalar }
}

impl MatchesTerminal for f64 {
    const DATA_TYPE: DataType = DataType::Float;
    fn get_shape(&self) -> Shape { Shape::Scalar }
}

impl MatchesTerminal for Vec<i64> {
    const DATA_TYPE: DataType = DataType::Integer;
    fn get_shape(&self) -> Shape { Shape::Vector(self.len()) }
}

impl MatchesTerminal for Vec<f64> {
    const DATA_TYPE: DataType = DataType::Float;
    fn get_shape(&self) -> Shape { Shape::Vector(self.len()) }
}

impl MatchesTerminal for Vec<Vec<i64>> {
    const DATA_TYPE: DataType = DataType::Integer;
    fn get_shape(&self) -> Shape { 
        if self.is_empty() {
            Shape::Matrix(0, 0)
        } else {
            Shape::Matrix(self.len(), self[0].len())
        }
    }
}

impl MatchesTerminal for Vec<Vec<f64>> {
    const DATA_TYPE: DataType = DataType::Float;
    fn get_shape(&self) -> Shape { 
         if self.is_empty() {
            Shape::Matrix(0, 0)
        } else {
            Shape::Matrix(self.len(), self[0].len())
        }
    }
}


impl Node {
    pub fn evaluate(&self, arena: &[Node]) -> Result<Box<dyn std::any::Any>, String> {
        self.evaluate_with_context(arena, &VariableContext::new())
    }

    pub fn evaluate_with_context(&self, arena: &[Node], context: &VariableContext) -> Result<Box<dyn std::any::Any>, String> {
        match self._type {
            NodeType::Terminal(data_type, shape) => {
                // Check if this terminal is a variable
                if let Some(variable_id) = &self.variable_id {
                    // Look up the variable value in the context
                    if let Some(var_value) = context.get_variable(variable_id) {
                        TypeRegistry::extract_terminal(var_value, data_type, shape)
                    } else {
                        Err(format!("Variable '{}' not found in context", variable_id))
                    }
                } else {
                    // Regular terminal with fixed value
                    TypeRegistry::extract_terminal(&self.value, data_type, shape)
                }
            }
            NodeType::NonTerminal(operation) => {
                let left_idx = self.left.ok_or("NonTerminal missing left child")?;
                let right_idx = self.right.ok_or("NonTerminal missing right child")?;
                
                let left_node = arena.get(left_idx).ok_or("Invalid left child index")?;
                let right_node = arena.get(right_idx).ok_or("Invalid right child index")?;
                
                let left_val = left_node.evaluate_with_context(arena, context)?;
                let right_val = right_node.evaluate_with_context(arena, context)?;
                
                let left_type = self.extract_type_info(&left_node._type)?;
                let right_type = self.extract_type_info(&right_node._type)?;
                
                TypeRegistry::execute_operation(operation, left_type, right_type, left_val, right_val)
            }
        }
    }

    fn extract_type_info(&self, node_type: &NodeType) -> Result<(DataType, Shape), String> {
        match node_type {
            NodeType::Terminal(data_type, shape) => Ok((*data_type, *shape)),
            NodeType::NonTerminal(_) => Err("Cannot extract type info from NonTerminal".to_string()),
        }
    }
       
    pub fn get_type(&self) -> NodeType {
        self._type
    }
    pub fn get_left_child_idx(&self) -> Option<usize> {
        self.left
    }
    pub fn get_right_child_idx(&self) -> Option<usize> {
        self.right
    }
    pub fn get_parent_idx(&self) -> usize {
        self.parent
    }

   pub fn new_terminal<T: 'static + MatchesTerminal>(
        idx: usize,
        variable_id: Option<String>,
        value: T,
        left: usize,
        right: usize,
        parent: usize,
    ) -> Self {
        let shape = value.get_shape();
        Node {
            idx,
            variable_id,
            _type: NodeType::Terminal(T::DATA_TYPE, shape),
            value: Box::new(value),
            left: Some(left),
            right: Some(right),
            parent,
        }
    }

    pub fn new_non_terminal<T: 'static>(
        idx: usize,
        variable_id: Option<String>,
        operation: Operation,
        value: T,
        left: usize,
        right: usize,
        parent: usize,
    ) -> Self {
        Node {
            idx,
            variable_id,
            _type: NodeType::NonTerminal(operation),
            value: Box::new(value),
            left: Some(left),
            right: Some(right),
            parent,
        }
    }
}