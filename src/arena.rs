use crate::types::{ DataType, Shape};
use crate::node::{Node, NodeType};
use crate::ops::{Operation};

use rand::Rng;
use std::any::Any;

enum GenerationMethod {
    Full,
    Grow
}
pub struct Arena{
    arena: Vec<Node>
}

impl Arena {
    pub fn init() -> Self {
        Arena {
            arena: Vec::new()
        }
    }
    pub fn init_with_method(method: GenerationMethod, max_depth: usize) -> Self {
        let mut arena = Arena { arena: Vec::new() };
        
        // Generate a root node using the specified method
        let root_index = arena.generate_tree(max_depth, &method);
        
        arena
    }

    fn generate_tree(&mut self, max_depth: usize, method: &GenerationMethod) -> usize {
        match method {
            GenerationMethod::Full => self.generate_full_tree(max_depth, 0),
            GenerationMethod::Grow => self.generate_grow_tree(max_depth, 0),
        }
    }

    fn generate_full_tree(&mut self, max_depth: usize, parent_idx: usize) -> usize {
        if max_depth == 1 {
            // At max depth, always generate a terminal
            self.add_random_terminal(parent_idx)
        } else {
            // For full method, always generate non-terminals until max depth
            let node_idx = self.add_random_non_terminal(parent_idx);
            
            // Generate left and right children
            let left_idx = self.generate_full_tree(max_depth - 1, node_idx);
            let right_idx = self.generate_full_tree(max_depth - 1, node_idx);
            
            // Set children indices
            if let Some(node) = self.arena.get_mut(node_idx) {
                node.left = left_idx;
                node.right = right_idx;
            }
            
            node_idx
        }
    }
    
    fn generate_grow_tree(&mut self, max_depth: usize, parent_idx: usize) -> usize {
        if max_depth == 1 {
            // At max depth, always generate a terminal
            self.add_random_terminal(parent_idx)
        } else {
            // For grow method, randomly choose between terminal and non-terminal
            let mut rng = rand::rng();
            
            if rng.random_bool(0.5) {  // 50% chance for terminal
                self.add_random_terminal(parent_idx)
            } else {
                // Generate a non-terminal
                let node_idx = self.add_random_non_terminal(parent_idx);
                
                // Generate left and right children
                let left_idx = self.generate_grow_tree(max_depth - 1, node_idx);
                let right_idx = self.generate_grow_tree(max_depth - 1, node_idx);
                
                // Set children indices
                if let Some(node) = self.arena.get_mut(node_idx) {
                    node.left = left_idx;
                    node.right = right_idx;
                }
                
                node_idx
            }
        }
    }
    
    fn add_random_terminal(&mut self, parent_idx: usize) -> usize {
        let mut rng = rand::rng();
        
        // Randomly choose a data type
        // probably should pull this from some registry..
        let data_type = if rng.random_bool(0.5) {
            DataType::Integer
        } else {
            DataType::Float
        };
        
        // Create the terminal node type
        let node_type = NodeType::Terminal(data_type, Shape::Scalar);
        
       let node = match data_type {
            DataType::Integer => {
                let value: i64 = rng.random_range(-100..100);
                Node::new_terminal(idx, name.clone(), value, left, right, parent)
            }
            DataType::Float => {
                let value: f64 = rng.random_range(-100.0..100.0);
                Node::new_terminal(idx, name.clone(), value, left, right, parent)
            }
        };

        // Add the node to the arena
        let idx = self.arena.len();
        self.arena.push(node);
        
        idx
    }
    
    fn add_random_non_terminal(&mut self, parent_idx: usize) -> usize {
        let mut rng = rand::rng();
        
        // Available operations
        let operations = [
            Operation::Add,
            Operation::Subtract,
            Operation::Multiply,
            Operation::Divide,
        ];
        
        // Randomly choose an operation
        let operation = operations[rng.random_range(0..operations.len())];
        
        // Create the non-terminal node type
        let node_type = NodeType::NonTerminal(operation);
        
        // Create a value representation for the operation (using String in this example)
        let value: Box<dyn Any> = match operation {
            Operation::Add => Box::new("add".to_string()),
            Operation::Subtract => Box::new("subtract".to_string()),
            Operation::Multiply => Box::new("multiply".to_string()),
            Operation::Divide => Box::new("divide".to_string()),
        };
        
        // Add the node to the arena
        let idx = self.arena.len();
        self.arena.push(Node {
            idx,
            _type: node_type,
            value,
            left: 0,
            right: 0,
            parent: parent_idx,
        });
        
        idx
    }
}