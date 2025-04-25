use std::fmt;
use std::rc::Rc;
use rand::prelude::*; // Import the necessary traits from rand

use super::terminal::{Terminal, TerminalShape, GenericFunction};
//// Types for the tree structure used in STGP

/// Generation method enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenerationMethod {
    Full,   // All branches have the same depth
    Grow,   // Branches can have different depths
}

/// Using a arena pattern, tree implemented as a vector.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GPTree {
    nodes: Vec<NodeData>,
    root_idx: usize,
    max_initial_tree_size: usize,
}

/// A node in the tree. Can be a Function or Terminal. Note that this only allows downward traversal of the tree.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NodeData {
    Terminal(Terminal),
    Function {
        function: GenericFunction,
        arguments: Vec<usize>, // Indices into the nodes vector
    }
}

impl GPTree {
    pub fn new() -> Self {
        GPTree {
            nodes: Vec::new(),
            root_idx: 0,
            max_initial_tree_size: 0
        }
    }

     /// Generate a random tree with the given return type
     pub fn generate_random(
        terminal_set: &[Terminal],
        function_set: &[GenericFunction],
        return_type: &TerminalShape,
        max_depth: usize,
        method: GenerationMethod,
    ) -> Result<Self, String> {
        let mut tree = GPTree {
            nodes: Vec::new(),
            root_idx: 0,
            max_initial_tree_size: max_depth 
        };
        
        // Generate the tree structure
        tree.root_idx = tree.generate_node(
            terminal_set,
            function_set,
            return_type,
            max_depth,
            method,
            1, // Start at depth 1
        )?;
        
        Ok(tree)
    }

    /// Generate a node with the given output type and add it to the tree
    fn generate_node(
        &mut self,
        terminal_set: &[Terminal],
        function_set: &[GenericFunction],
        output_type: &TerminalShape,
        max_depth: usize,
        method: GenerationMethod,
        current_depth: usize,
    ) -> Result<usize, String> {
        // Must use terminal if at max depth
        if current_depth >= max_depth {
            return self.create_random_terminal(terminal_set, output_type);
        }
        
        // Decide whether to use terminal or function based on method
        let use_function = match method {
            GenerationMethod::Full => current_depth < max_depth - 1,
            GenerationMethod::Grow => current_depth < max_depth - 1 && rand::random::<bool>(),
        };
        
        if use_function {
            // Try to create a function node
            match self.create_random_function(
                terminal_set, 
                function_set, 
                output_type,
                max_depth,
                method,
                current_depth,
            ) {
                Ok(idx) => Ok(idx),
                Err(_) => {
                    // Fall back to terminal if no compatible function
                    self.create_random_terminal(terminal_set, output_type)
                }
            }
        } else {
            // Create a terminal node
            self.create_random_terminal(terminal_set, output_type)
        }
    }

        /// Create a random terminal node of the given type
        fn create_random_terminal(
            &mut self,
            terminal_set: &[Terminal],
            output_type: &TerminalShape,
        ) -> Result<usize, String> {
            // Find all terminals of compatible type
            let compatible_terminals: Vec<_> = terminal_set.iter()
                .filter(|terminal| {
                    let terminal_type = match terminal {
                        Terminal::Variable(shape) | Terminal::Constant(shape) => shape,
                    };
                    is_type_compatible(terminal_type, output_type)
                })
                .collect();
            
            if compatible_terminals.is_empty() {
                return Err(format!("No compatible terminals found for type {:?}", output_type));
            }
            
            // Choose a random compatible terminal
            let terminal = compatible_terminals
                .choose(&mut rand::rng())
                .ok_or("Failed to select terminal")?
                .clone();
            
            // Add the terminal to the tree
            let idx = self.nodes.len();
            self.nodes.push(NodeData::Terminal(*terminal));
            
            Ok(idx)
        }
        
        /// Create a random function node of the given type
        fn create_random_function(
            &mut self,
            terminal_set: &[Terminal],
            function_set: &[GenericFunction],
            output_type: &TerminalShape,
            max_depth: usize,
            method: GenerationMethod,
            current_depth: usize,
        ) -> Result<usize, String> {
            // Find all functions that return the right type
            let compatible_functions: Vec<_> = function_set.iter()
                .filter(|function| is_type_compatible(&function.output, output_type))
                .collect();
            
            if compatible_functions.is_empty() {
                return Err(format!("No compatible functions found for type {:?}", output_type));
            }
            
            // Choose a random compatible function
            let function = compatible_functions
                .choose(&mut rand::rng())
                .ok_or("Failed to select function")?
                .clone();
            
            // Generate arguments recursively
            let mut argument_indices = Vec::new();
            for input_type in &function.inputs {
                let arg_idx = self.generate_node(
                    terminal_set,
                    function_set,
                    input_type,
                    max_depth,
                    method,
                    current_depth + 1,
                )?;
                argument_indices.push(arg_idx);
            }
            
            // Add the function node to the tree
            let idx = self.nodes.len();
            self.nodes.push(NodeData::Function {
                function: *function,
                arguments: argument_indices,
            });
            
            Ok(idx)
        }
        
        /// Get the output type of a node
        pub fn get_node_type(&self, idx: usize) -> TerminalShape {
            match &self.nodes[idx] {
                NodeData::Terminal(terminal) => {
                    match terminal {
                        Terminal::Variable(shape) | Terminal::Constant(shape) => shape.clone(),
                    }
                },
                NodeData::Function { function, .. } => function.output.clone(),
            }
        }
}

