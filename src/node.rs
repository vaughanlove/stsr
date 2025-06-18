use crate::ops::Operation;
use crate::types::{DataType, Shape, TypeInfo};
// use crate::registry::TypeRegistry;
use crate::variable::VariableContext;
use std::any::Any;

type InputOneType = TypeInfo;
type InputTwoType = TypeInfo;
type OutputType = TypeInfo;

#[derive(Clone, Copy, Debug)]
pub enum NodeType {
    // Heap-allocated NonTerminal
    /// input type, input type, operation, output type
    NonTerminal(InputOneType, InputTwoType, Operation, OutputType),

    /// Heap-allocated Terminal
    Terminal(TypeInfo),
}

/// Given two inputs and an operation, return possible output types
pub fn compatible_outputs(input1: TypeInfo, input2: TypeInfo, op: Operation) -> Vec<TypeInfo> {
    use crate::types::{DataType, Shape};
    
    match (input1.shape, input2.shape, op, input1.data_type == input2.data_type) {
        // Scalar + Scalar
        (Shape::Scalar, Shape::Scalar, _, true) => {
            vec![TypeInfo { data_type: input1.data_type, shape: Shape::Scalar }]
        },
        
        // Vector + Vector (same size)
        (Shape::Vector(n1), Shape::Vector(n2), _, true) if n1 == n2 => {
            vec![TypeInfo { data_type: input1.data_type, shape: Shape::Vector(n1) }]
        },
        
        // Vector + Scalar broadcasting
        (Shape::Vector(n), Shape::Scalar, _, true) | (Shape::Scalar, Shape::Vector(n), _, true) => {
            vec![TypeInfo { data_type: input1.data_type, shape: Shape::Vector(n) }]
        },
        
        // Matrix + Matrix
        (Shape::Matrix(r1, c1), Shape::Matrix(r2, c2), Operation::Add | Operation::Subtract, true) 
            if r1 == r2 && c1 == c2 => {
            vec![TypeInfo { data_type: input1.data_type, shape: Shape::Matrix(r1, c1) }]
        },
        
        // Matrix multiplication
        (Shape::Matrix(m, n1), Shape::Matrix(n2, p), Operation::Multiply, true) if n1 == n2 => {
            vec![TypeInfo { data_type: input1.data_type, shape: Shape::Matrix(m, p) }]
        },
        
        _ => vec![],
    }
}

/// Given an operation and desired output, return possible input pairs
// pub fn compatible_inputs(op: Operation, output: TypeInfo) -> Vec<(TypeInfo, TypeInfo)> {
//     use crate::types::{DataType, Shape};
    
//     let mut inputs = Vec::new();
    
//     match (op, output.shape) {
//         // Scalar output
//         (_, Shape::Scalar) => {
//             // Scalar + Scalar -> Scalar
//             let scalar_type = TypeInfo { _type: output._type, shape: Shape::Scalar };
//             inputs.push((scalar_type, scalar_type));
//         },
        
//         // Vector output
//         (_, Shape::Vector(n)) => {
//             let vector_type = TypeInfo { _type: output._type, shape: Shape::Vector(n) };
//             let scalar_type = TypeInfo { _type: output._type, shape: Shape::Scalar };
            
//             // Vector + Vector -> Vector
//             inputs.push((vector_type, vector_type));
//             // Vector + Scalar -> Vector (broadcasting)
//             inputs.push((vector_type, scalar_type));
//             inputs.push((scalar_type, vector_type));
//         },
        
//         // Matrix output
//         (Operation::Add | Operation::Subtract, Shape::Matrix(r, c)) => {
//             let matrix_type = TypeInfo { _type: output._type, shape: Shape::Matrix(r, c) };
//             // Matrix + Matrix -> Matrix (element-wise)
//             inputs.push((matrix_type, matrix_type));
//         },
        
//         (Operation::Multiply, Shape::Matrix(m, p)) => {
//             // For matrix multiplication, we need to generate all valid (m,k) * (k,p) combinations
//             // This is a simplification - in practice you'd iterate over reasonable k values
//             for k in 1..=10 { // arbitrary limit for example
//                 let left_matrix = TypeInfo { _type: output._type, shape: Shape::Matrix(m, k) };
//                 let right_matrix = TypeInfo { _type: output._type, shape: Shape::Matrix(k, p) };
//                 inputs.push((left_matrix, right_matrix));
//             }
//         },
        
//         _ => {},
//     }
    
//     inputs
// }

#[derive(Debug)]
pub struct Node {
    pub idx: usize,
    pub _type: NodeType, // for GPSR
    pub value: Box<dyn Any>,
    pub variable_id: Option<String>, // for generics that need to pull value from variable.rs HashMap.
    pub left_index: Option<usize>,
    pub right_index: Option<usize>,
    pub parent_index: usize,
}

// pub trait MatchesTerminal {
//     const DATA_TYPE: DataType;
//     fn get_shape(&self) -> Shape; 
// }

// impl MatchesTerminal for i64 {
//     const DATA_TYPE: DataType = DataType::Integer;
//     fn get_shape(&self) -> Shape { Shape::Scalar }
// }

// impl MatchesTerminal for f64 {
//     const DATA_TYPE: DataType = DataType::Float;
//     fn get_shape(&self) -> Shape { Shape::Scalar }
// }

// impl MatchesTerminal for Vec<i64> {
//     const DATA_TYPE: DataType = DataType::Integer;
//     fn get_shape(&self) -> Shape { Shape::Vector(self.len()) }
// }

// impl MatchesTerminal for Vec<f64> {
//     const DATA_TYPE: DataType = DataType::Float;
//     fn get_shape(&self) -> Shape { Shape::Vector(self.len()) }
// }

// impl MatchesTerminal for Vec<Vec<i64>> {
//     const DATA_TYPE: DataType = DataType::Integer;
//     fn get_shape(&self) -> Shape { 
//         if self.is_empty() {
//             Shape::Matrix(0, 0)
//         } else {
//             Shape::Matrix(self.len(), self[0].len())
//         }
//     }
// }

// impl MatchesTerminal for Vec<Vec<f64>> {
//     const DATA_TYPE: DataType = DataType::Float;
//     fn get_shape(&self) -> Shape { 
//          if self.is_empty() {
//             Shape::Matrix(0, 0)
//         } else {
//             Shape::Matrix(self.len(), self[0].len())
//         }
//     }
// }


impl Node {
//     pub fn evaluate(&self, arena: &[Node]) -> Result<Box<dyn std::any::Any>, String> {
//         self.evaluate_with_context(arena, &VariableContext::new())
//     }

//     pub fn evaluate_with_context(&self, arena: &[Node], context: &VariableContext) -> Result<Box<dyn std::any::Any>, String> {
//         match self._type {
//             NodeType::Terminal(_type) => {
//                 // Check if this terminal is a variable
//                 if let Some(variable_id) = &self.variable_id {
//                     // Look up the variable value in the context
//                     if let Some(var_value) = context.get_variable(variable_id) {
//                         TypeRegistry::extract_terminal(var_value, data_type, shape)
//                     } else {
//                         Err(format!("Variable '{}' not found in context", variable_id))
//                     }
//                 } else {
//                     // Regular terminal with fixed value
//                     TypeRegistry::extract_terminal(&self.value, data_type, shape)
//                 }
//             }
//             NodeType::NonTerminal(operation) => {
//                 let left_idx = self.left.ok_or("NonTerminal missing left child")?;
//                 let right_idx = self.right.ok_or("NonTerminal missing right child")?;
                
//                 let left_node = arena.get(left_idx).ok_or("Invalid left child index")?;
//                 let right_node = arena.get(right_idx).ok_or("Invalid right child index")?;
                
//                 let left_val = left_node.evaluate_with_context(arena, context)?;
//                 let right_val = right_node.evaluate_with_context(arena, context)?;
                
//                 let left_type = self.extract_type_info(&left_node._type)?;
//                 let right_type = self.extract_type_info(&right_node._type)?;
                
//                 TypeRegistry::execute_operation(operation, left_type, right_type, left_val, right_val)
//             }
//         }
//     }

//     fn extract_type_info(&self, node_type: &NodeType) -> Result<(DataType, Shape), String> {
//         match node_type {
//             NodeType::Terminal(data_type, shape) => Ok((*data_type, *shape)),
//             NodeType::NonTerminal(_) => Err("Cannot extract type info from NonTerminal".to_string()),
//         }
//     }
       
//     pub fn get_type(&self) -> NodeType {
//         self._type
//     }
//     pub fn get_left_child_idx(&self) -> Option<usize> {
//         self.left
//     }
//     pub fn get_right_child_idx(&self) -> Option<usize> {
//         self.right
//     }
//     pub fn get_parent_idx(&self) -> usize {
//         self.parent
//     }

//    pub fn new_terminal<T: 'static + MatchesTerminal>(
//         idx: usize,
//         variable_id: Option<String>,
//         value: T,
//         left: usize,
//         right: usize,
//         parent: usize,
//     ) -> Self {
//         let shape = value.get_shape();
//         Node {
//             idx,
//             variable_id,
//             _type: NodeType::Terminal(T::DATA_TYPE, shape),
//             value: Box::new(value),
//             left: Some(left),
//             right: Some(right),
//             parent,
//         }
//     }

    pub fn new_non_terminal<T: 'static>(
        idx: usize,
        variable_id: Option<String>,
        operation: Operation,
        value: T,
        left_type: TypeInfo,
        right_type: TypeInfo,
        output_type: TypeInfo,
        left_index: usize,
        right_index: usize,
        parent_index: usize,
    ) -> Self {
        Node {
            idx,
            variable_id,
            _type: NodeType::NonTerminal(left_type, right_type, operation, output_type),
            value: Box::new(value),
            left_index: Some(left_index),
            right_index: Some(right_index),
            parent_index,
        }
    }
}