use crate::ops::Operation;
use crate::types::{DataType, Shape};
use std::any::Any;

#[derive(Clone, Copy)]
pub enum NodeType {
    // Heap-allocated NonTerminal
    // input type, input type, operation, output type
    NonTerminal(Operation),

    // Heap-allocated Terminal
    Terminal(DataType, Shape),
}

pub struct Node {
    pub idx: usize,
    pub name: String, // for generics that need to pull value from variable.rs HashMap.
    pub _type: NodeType, // for GPSR
    pub value: Box<dyn Any>,
    pub left: usize,
    pub right: usize,
    pub parent: usize,
}

trait MatchesTerminal {
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
    const DATA_TYPE: DataType = DataType::Float;
    fn get_shape(&self) -> Shape { Shape::Vector(self.len()) }
}

impl MatchesTerminal for Vec<f64> {
    const DATA_TYPE: DataType = DataType::Float;
    fn get_shape(&self) -> Shape { Shape::Vector(self.len()) }
}

impl MatchesTerminal for Vec<Vec<i64>> {
    const DATA_TYPE: DataType = DataType::Float;
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
    pub fn get_value(&self) -> &dyn Any {
        self.value.as_ref()
    }
    pub fn get_type(&self) -> NodeType {
        self._type
    }
    pub fn get_left_child_idx(&self) -> usize {
        self.left
    }
    pub fn get_right_child_idx(&self) -> usize {
        self.right
    }
    pub fn get_parent_idx(&self) -> usize {
        self.parent
    }

   pub fn new_terminal<T: 'static + MatchesTerminal>(
        idx: usize,
        name: String,
        value: T,
        left: usize,
        right: usize,
        parent: usize,
    ) -> Self {
        let shape = value.get_shape();

        Node {
            idx,
            name,
            _type: NodeType::Terminal(T::DATA_TYPE, shape),
            value: Box::new(value),
            left,
            right,
            parent,
        }
    }

    pub fn new_non_terminal<T: 'static>(
        idx: usize,
        name: String,
        operation: Operation,
        value: T,
        left: usize,
        right: usize,
        parent: usize,
    ) -> Self {
        Node {
            idx,
            name,
            _type: NodeType::NonTerminal(operation),
            value: Box::new(value),
            left,
            right,
            parent,
        }
    }
}