// use crate::shape::ShapeGrammar;
// use crate::types::{DataType, Shape, TypeInfo};
// use crate::node::{Node, NodeType};
// use crate::ops::Operation;
// use crate::registry::TypeRegistry;
// use crate::types::Variable;
// use rand::Rng;
// use std::collections::HashSet;



// /// In GPSR, there are two generation methods outlined.
// /// Grow - Terminals and Nonterminals can appear at any depth - randomly chosen during construction. Leaves are always Terminals.
// /// Full - The entire tree is filled up until max_depth - 1 with NonTerminals. The leaves are then all populated with Terminals. 
// #[derive(Clone, Copy, Debug)]
// pub enum GenerationMethod {
//     Full,
//     Grow,
// }

// /// Arena implementation for representing the Nodes in a tree structure.
// #[derive(Debug)]
// pub struct Arena {
//     arena: Vec<Node>,
//     grammar: ShapeGrammar
// }

// impl Arena {
//     /// Initialize an empty arena.
//     pub fn init(grammar: ShapeGrammar) -> Self {
//         Arena {
//             arena: Vec::new(),
//             grammar: grammar
//         }
//     }

//     /// Initialize an arena using the Full method or (eventually) the Grow method.
//     /// 
//     /// # Arguments
//     /// 
//     /// * `max_depth` - The maximum size of the tree
//     /// * `target_type` - The root is the eventual output of the function created. 
//     ///   `target_type` is the DataType you want as output
//     /// * `target_shape` - The target shape of the function's output
//     /// * `required_variables` - The variables that you are providing in the input dataset
//     /// 
//     /// # Returns
//     /// 
//     /// Returns an `Arena` instance
//     pub fn init_with_method(
//         method: GenerationMethod,
//         max_depth: usize,
//         target_type: DataType,
//         target_shape: Shape,
//         required_variables: Vec<Variable>,
//         grammar: ShapeGrammar
//     ) -> Result<Self, String> {
//         let mut arena = Arena {
//             arena: Vec::new(),
//             grammar: grammar,
//         };

//         // Validate that tree can accommodate all required variables
//         let max_terminals = 2_usize.pow(max_depth as u32 - 1);
//         let compatible_var_count = required_variables
//             .iter()
//             .filter(|var| var.data_type == target_type && var.shape == target_shape)
//             .count();
        
//         if compatible_var_count > max_terminals {
//             return Err(format!(
//                 "Cannot fit {} compatible variables in tree with {} terminal slots",
//                 compatible_var_count, max_terminals
//             ));
//         }

//         // Generate a root node using the specified method that returns the target type
//         let remaining_vars = arena.generate_tree_with_variables(max_depth, &method, target_type, target_shape, required_variables.clone(), 0);

//         // Verify all required variables were placed
//         if !remaining_vars.is_empty() {
//             return Err(format!(
//                 "Failed to place required variables: {:?}",
//                 remaining_vars.iter().map(|v| &v.name).collect::<Vec<_>>()
//             ));
//         }

//         arena.validate_required_variables(&required_variables)?;

//         Ok(arena)
//     }

//     /// Return all of the nodes in the arena
//     pub fn get_nodes(&self) -> &[Node] {
//         &self.arena
//     }

//     /// Return a reference to the first node in the arena.
//     pub fn get_root(&self) -> Option<&Node> {
//         self.arena.first()
//     }

//     fn generate_tree_with_variables(
//         &mut self,
//         max_depth: usize,
//         method: &GenerationMethod,
//         required_type: DataType,
//         required_shape: Shape,
//         required_variables: Vec<Variable>,
//         parent_idx: usize,
//     ) -> Vec<Variable> {
//         match method {
//             GenerationMethod::Full => {
//                 self.generate_full_tree_with_variables(max_depth, required_type, required_shape, required_variables, parent_idx).1
//             }
//             GenerationMethod::Grow => {
//                 // self.generate_grow_tree_with_variables(max_depth, required_type, required_shape, required_variables, parent_idx)
//                 unimplemented!()
//             }
//         }
//     }

//     fn add_terminal(&mut self, data_type: DataType, shape: Shape, parent_idx: usize) -> usize {
//         let idx = self.arena.len();
//         let value = TypeRegistry::create_random_terminal(data_type, shape);

//         let node = Node {
//             idx,
//             variable_id: None,
//             _type: NodeType::Terminal(TypeInfo {_type: data_type, shape: shape} ),
//             value,
//             left: None,
//             right: None,
//             parent: parent_idx,
//         };

//         self.arena.push(node);
//         idx
//     }

//     fn add_variable_terminal(&mut self, data_type: DataType, shape: Shape, variable_id: String, parent_idx: usize) -> usize {
//         let idx = self.arena.len();
//         let value = TypeRegistry::create_variable_terminal(data_type, shape);

//         let node = Node {
//             idx,
//             variable_id: Some(variable_id),
//             _type: NodeType::Terminal(TypeInfo {_type: data_type, shape: shape} ),
//             value,
//             left: None,
//             right: None,
//             parent: parent_idx,
//         };

//         self.arena.push(node);
//         idx
//     }

//     fn add_non_terminal(
//         &mut self,
        
//         parent_idx: usize,
//     ) -> usize {
//         let mut rng = rand::rng();
//         let idx = self.arena.len();

//         // Available operations - all return the same type as their operands
//         let operations = [
//             Operation::Add,
//             Operation::Subtract,
//             Operation::Multiply,
//             Operation::Divide,
//         ];

//         let operation = operations[rng.random_range(0..operations.len())];

//         // Non-terminals store a string representation for debugging
//         let op_name = match operation {
//             Operation::Add => "add",
//             Operation::Subtract => "subtract",
//             Operation::Multiply => "multiply",
//             Operation::Divide => "divide",
//         };

//         let node = Node::new_non_terminal(
//             idx,
//             None,
//             operation,
//             op_name.to_string(),
//             left_type: 
//             0, // Will be set when children are generated
//             0, // Will be set when children are generated
//             parent_idx,
//         );

//         self.arena.push(node);
//         idx
//     }

//     fn generate_full_tree_with_variables(
//         &mut self,
//         max_depth: usize,
//         required_type: DataType,
//         required_shape: Shape,
//         mut required_variables: Vec<Variable>,
//         parent_idx: usize,
//     ) -> (usize, Vec<Variable>) {
//         // FUTURE: make this a lookup table accessible in the arena object instead of instantiating as a vector each iteration.
//         let compatible_shapes = self.grammar.compatible_shapes(&required_shape);

//         println!("The compatible shapes for the root terminal: {:?}", compatible_shapes);

//         // Filter variables that match the current type requirements
//         let compatible_vars: Vec<Variable> = required_variables
//             .iter()
//             .filter(|var| var.data_type == required_type && var.shape == required_shape)
//             .cloned().collect();


//         if max_depth == 1 {
//             // At max depth, must generate a terminal
//             if !compatible_vars.is_empty() {
//                 // Prioritize using a required variable at terminal positions
//                 let var = compatible_vars.into_iter().next().unwrap();
//                 // Remove this variable from the required list since it's now placed
//                 required_variables.retain(|v| v.name != var.name);

//                 let node_idx = self.add_variable_terminal(var.data_type, var.shape, var.name.clone(), parent_idx);
//                 (node_idx, required_variables)
//             } else {
//                 // Generate regular terminal only if no compatible variables remain
//                 let node_idx = self.add_terminal(required_type, required_shape, parent_idx);
//                 (node_idx, required_variables)
//             }
//         } else {
//             // Generate non-terminal and recurse
//             let node_idx = self.add_non_terminal(required_type, required_shape, parent_idx);

//             // Strategic distribution: ensure variables get placed
//             let (left_vars, right_vars) = self.distribute_variables_strategically(required_variables, max_depth - 1, required_type, required_shape);

//             let (left_idx, mut remaining_vars) = self.generate_full_tree_with_variables(max_depth - 1, required_type, required_shape, left_vars, node_idx);
//             let (right_idx, right_remaining) = self.generate_full_tree_with_variables(max_depth - 1, required_type, required_shape, right_vars, node_idx);

//             // Combine remaining variables from both subtrees
//             remaining_vars.extend(right_remaining);

//             // Set children indices
//             if let Some(node) = self.arena.get_mut(node_idx) {
//                 node.left = Some(left_idx);
//                 node.right = Some(right_idx);
//             }

//             (node_idx, remaining_vars)
//         }
//     }

//     fn distribute_variables_strategically(&self, mut variables: Vec<Variable>, _remaining_depth: usize, required_type: DataType, required_shape: Shape) -> (Vec<Variable>, Vec<Variable>) {
//         let mut left_vars = Vec::new();
//         let mut right_vars = Vec::new();

//         // Filter compatible variables for this subtree
//         let compatible_vars: Vec<Variable> = variables
//             .iter()
//             .filter(|var| var.data_type == required_type && var.shape == required_shape)
//             .cloned()
//             .collect();

//         // Note: Each subtree in Full method has 2^(depth-1) terminal slots available
        
//         // Ensure both subtrees get a fair distribution of compatible variables
//         let compatible_count = compatible_vars.len();
//         if compatible_count > 0 {
//             let left_allocation = (compatible_count + 1) / 2; // Ceiling division
            
//             // Distribute compatible variables
//             for (i, var) in compatible_vars.into_iter().enumerate() {
//                 if i < left_allocation {
//                     left_vars.push(var.clone());
//                 } else {
//                     right_vars.push(var.clone());
//                 }
//                 // Remove from original list
//                 variables.retain(|v| v.name != var.name);
//             }
//         }

//         // Distribute remaining incompatible variables randomly
//         let mut rng = rand::rng();
//         for var in variables.drain(..) {
//             if rng.random_bool(0.5) {
//                 left_vars.push(var);
//             } else {
//                 right_vars.push(var);
//             }
//         }

//         (left_vars, right_vars)
//     }


//     pub fn validate_required_variables(&self, required_variables: &[Variable]) -> Result<(), String> {
//         let mut found_variables = HashSet::new();
        
//         // Collect all variable names present in the tree
//         for node in &self.arena {
//             if let Some(var_id) = &node.variable_id {
//                 found_variables.insert(var_id.clone());
//             }
//         }

//         // Check if all required variables are present
//         for required_var in required_variables {
//             if !found_variables.contains(&required_var.name) {
//                 return Err(format!("Required variable '{}' not found in generated tree", required_var.name));
//             }
//         }

//         Ok(())
//     }

//     pub fn get_variable_names(&self) -> Vec<String> {
//         self.arena
//             .iter()
//             .filter_map(|node| node.variable_id.clone())
//             .collect()
//     }
// }