use crate::types::{DataType, Shape};
use crate::node::{Node, NodeType};
use crate::ops::Operation;
use crate::registry::TypeRegistry;
use crate::variable::Variable;
use rand::Rng;
use std::collections::HashSet;

#[derive(Clone, Copy, Debug)]
pub enum GenerationMethod {
    Full,
    Grow,
}

pub struct Arena {
    arena: Vec<Node>,
}

impl Arena {
    pub fn init() -> Self {
        Arena {
            arena: Vec::new(),
        }
    }

    pub fn init_with_method(
        method: GenerationMethod,
        max_depth: usize,
        target_type: DataType,
        target_shape: Shape,
        required_variables: Vec<Variable>,
    ) -> Self {
        let mut arena = Arena {
            arena: Vec::new(),
        };

        // Generate a root node using the specified method that returns the target type
        arena.generate_tree_with_variables(max_depth, &method, target_type, target_shape, required_variables, 0);

        arena
    }

    pub fn get_nodes(&self) -> &[Node] {
        &self.arena
    }

    pub fn get_root(&self) -> Option<&Node> {
        self.arena.last()
    }

    fn generate_tree_with_variables(
        &mut self,
        max_depth: usize,
        method: &GenerationMethod,
        required_type: DataType,
        required_shape: Shape,
        required_variables: Vec<Variable>,
        parent_idx: usize,
    ) -> usize {
        match method {
            GenerationMethod::Full => {
                self.generate_full_tree_with_variables(max_depth, required_type, required_shape, required_variables, parent_idx)
            }
            GenerationMethod::Grow => {
                self.generate_grow_tree_with_variables(max_depth, required_type, required_shape, required_variables, parent_idx)
            }
        }
    }

    fn generate_tree(
        &mut self,
        max_depth: usize,
        method: &GenerationMethod,
        required_type: DataType,
        required_shape: Shape,
        parent_idx: usize,
    ) -> usize {
        self.generate_tree_with_variables(max_depth, method, required_type, required_shape, Vec::new(), parent_idx)
    }

    fn generate_full_tree(
        &mut self,
        max_depth: usize,
        required_type: DataType,
        required_shape: Shape,
        parent_idx: usize,
    ) -> usize {
        if max_depth == 1 {
            // At max depth, always generate a terminal of the required type
            self.add_terminal(required_type, required_shape, parent_idx)
        } else {
            // For full method, always generate non-terminals until max depth
            let node_idx = self.add_non_terminal(required_type, required_shape, parent_idx);

            // Generate children - both operands must be the same type for our basic operations
            let left_idx = self.generate_full_tree(max_depth - 1, required_type, required_shape, node_idx);
            let right_idx = self.generate_full_tree(max_depth - 1, required_type, required_shape, node_idx);

            // Set children indices
            if let Some(node) = self.arena.get_mut(node_idx) {
                node.left = Some(left_idx);
                node.right = Some(right_idx);
            }

            node_idx
        }
    }

    fn generate_grow_tree(
        &mut self,
        max_depth: usize,
        required_type: DataType,
        required_shape: Shape,
        parent_idx: usize,
    ) -> usize {
        if max_depth == 1 {
            // At max depth, always generate a terminal
            self.add_terminal(required_type, required_shape, parent_idx)
        } else {
            // For grow method, randomly choose between terminal and non-terminal
            let mut rng = rand::thread_rng();

            if rng.gen_bool(0.5) {
                // 50% chance for terminal
                self.add_terminal(required_type, required_shape, parent_idx)
            } else {
                // Generate a non-terminal
                let node_idx = self.add_non_terminal(required_type, required_shape, parent_idx);

                // Generate children
                let left_idx = self.generate_grow_tree(max_depth - 1, required_type, required_shape, node_idx);
                let right_idx = self.generate_grow_tree(max_depth - 1, required_type, required_shape, node_idx);

                // Set children indices
                if let Some(node) = self.arena.get_mut(node_idx) {
                    node.left = Some(left_idx);
                    node.right = Some(right_idx);
                }

                node_idx
            }
        }
    }

    fn add_terminal(&mut self, data_type: DataType, shape: Shape, parent_idx: usize) -> usize {
        let idx = self.arena.len();
        let value = TypeRegistry::create_random_terminal(data_type, shape);

        let node = Node {
            idx,
            variable_id: None,
            _type: NodeType::Terminal(data_type, shape),
            value,
            left: None,
            right: None,
            parent: parent_idx,
        };

        self.arena.push(node);
        idx
    }

    fn add_variable_terminal(&mut self, data_type: DataType, shape: Shape, variable_id: String, parent_idx: usize) -> usize {
        let idx = self.arena.len();
        let value = TypeRegistry::create_variable_terminal(data_type, shape);

        let node = Node {
            idx,
            variable_id: Some(variable_id),
            _type: NodeType::Terminal(data_type, shape),
            value,
            left: None,
            right: None,
            parent: parent_idx,
        };

        self.arena.push(node);
        idx
    }

    fn add_non_terminal(
        &mut self,
        _required_type: DataType,
        _required_shape: Shape,
        parent_idx: usize,
    ) -> usize {
        let mut rng = rand::thread_rng();
        let idx = self.arena.len();

        // Available operations - all return the same type as their operands
        let operations = [
            Operation::Add,
            Operation::Subtract,
            Operation::Multiply,
            Operation::Divide,
        ];

        let operation = operations[rng.gen_range(0..operations.len())];

        // Non-terminals store a string representation for debugging
        let op_name = match operation {
            Operation::Add => "add",
            Operation::Subtract => "subtract",
            Operation::Multiply => "multiply",
            Operation::Divide => "divide",
        };

        let node = Node::new_non_terminal(
            idx,
            None,
            operation,
            op_name.to_string(),
            0, // Will be set when children are generated
            0, // Will be set when children are generated
            parent_idx,
        );

        self.arena.push(node);
        idx
    }

    fn generate_full_tree_with_variables(
        &mut self,
        max_depth: usize,
        required_type: DataType,
        required_shape: Shape,
        mut required_variables: Vec<Variable>,
        parent_idx: usize,
    ) -> usize {
        // Filter variables that match the current type requirements
        let compatible_vars: Vec<Variable> = required_variables
            .iter()
            .filter(|var| var.data_type == required_type && var.shape == required_shape)
            .cloned()
            .collect();

        if max_depth == 1 {
            // At max depth, must generate a terminal
            if !compatible_vars.is_empty() && rand::thread_rng().gen_bool(0.5) {
                // 50% chance to use a required variable if available
                let var = compatible_vars.into_iter().next().unwrap();
                // Remove this variable from the required list
                required_variables.retain(|v| v.name != var.name);
                self.add_variable_terminal(var.data_type, var.shape, var.name, parent_idx)
            } else {
                // Generate regular terminal
                self.add_terminal(required_type, required_shape, parent_idx)
            }
        } else {
            // Generate non-terminal and recurse
            let node_idx = self.add_non_terminal(required_type, required_shape, parent_idx);

            // Distribute remaining required variables between children
            let (left_vars, right_vars) = self.split_required_variables(required_variables);

            let left_idx = self.generate_full_tree_with_variables(max_depth - 1, required_type, required_shape, left_vars, node_idx);
            let right_idx = self.generate_full_tree_with_variables(max_depth - 1, required_type, required_shape, right_vars, node_idx);

            // Set children indices
            if let Some(node) = self.arena.get_mut(node_idx) {
                node.left = Some(left_idx);
                node.right = Some(right_idx);
            }

            node_idx
        }
    }

    fn generate_grow_tree_with_variables(
        &mut self,
        max_depth: usize,
        required_type: DataType,
        required_shape: Shape,
        mut required_variables: Vec<Variable>,
        parent_idx: usize,
    ) -> usize {
        // Filter variables that match the current type requirements
        let compatible_vars: Vec<Variable> = required_variables
            .iter()
            .filter(|var| var.data_type == required_type && var.shape == required_shape)
            .cloned()
            .collect();

        if max_depth == 1 {
            // At max depth, must generate a terminal
            if !compatible_vars.is_empty() && rand::thread_rng().gen_bool(0.5) {
                let var = compatible_vars.into_iter().next().unwrap();
                required_variables.retain(|v| v.name != var.name);
                self.add_variable_terminal(var.data_type, var.shape, var.name, parent_idx)
            } else {
                self.add_terminal(required_type, required_shape, parent_idx)
            }
        } else {
            let mut rng = rand::thread_rng();

            // If we have required variables and limited depth, bias toward non-terminals
            let force_nonterminal = !required_variables.is_empty() && max_depth <= 3;
            
            if force_nonterminal || rng.gen_bool(0.5) {
                if !compatible_vars.is_empty() && rng.gen_bool(0.3) {
                    // 30% chance to use a required variable
                    let var = compatible_vars.into_iter().next().unwrap();
                    required_variables.retain(|v| v.name != var.name);
                    self.add_variable_terminal(var.data_type, var.shape, var.name, parent_idx)
                } else {
                    // Generate a non-terminal
                    let node_idx = self.add_non_terminal(required_type, required_shape, parent_idx);

                    let (left_vars, right_vars) = self.split_required_variables(required_variables);

                    let left_idx = self.generate_grow_tree_with_variables(max_depth - 1, required_type, required_shape, left_vars, node_idx);
                    let right_idx = self.generate_grow_tree_with_variables(max_depth - 1, required_type, required_shape, right_vars, node_idx);

                    if let Some(node) = self.arena.get_mut(node_idx) {
                        node.left = Some(left_idx);
                        node.right = Some(right_idx);
                    }

                    node_idx
                }
            } else {
                // Generate terminal
                if !compatible_vars.is_empty() && rng.gen_bool(0.5) {
                    let var = compatible_vars.into_iter().next().unwrap();
                    required_variables.retain(|v| v.name != var.name);
                    self.add_variable_terminal(var.data_type, var.shape, var.name, parent_idx)
                } else {
                    self.add_terminal(required_type, required_shape, parent_idx)
                }
            }
        }
    }

    fn split_required_variables(&self, mut variables: Vec<Variable>) -> (Vec<Variable>, Vec<Variable>) {
        let mut rng = rand::thread_rng();
        let mut left_vars = Vec::new();
        let mut right_vars = Vec::new();

        // Randomly distribute variables between left and right subtrees
        for var in variables.drain(..) {
            if rng.gen_bool(0.5) {
                left_vars.push(var);
            } else {
                right_vars.push(var);
            }
        }

        (left_vars, right_vars)
    }

    pub fn validate_required_variables(&self, required_variables: &[Variable]) -> Result<(), String> {
        let mut found_variables = HashSet::new();
        
        // Collect all variable names present in the tree
        for node in &self.arena {
            if let Some(var_id) = &node.variable_id {
                found_variables.insert(var_id.clone());
            }
        }

        // Check if all required variables are present
        for required_var in required_variables {
            if !found_variables.contains(&required_var.name) {
                return Err(format!("Required variable '{}' not found in generated tree", required_var.name));
            }
        }

        Ok(())
    }

    pub fn get_variable_names(&self) -> Vec<String> {
        self.arena
            .iter()
            .filter_map(|node| node.variable_id.clone())
            .collect()
    }
}