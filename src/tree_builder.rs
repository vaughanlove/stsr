//! Tree building utilities and relating data structures.
//!
//! This module provides functionality to create trees given a Non-terminal Grammar. It provides the ability to build possibility tables
//! from this grammar. Note that in Montana's 1995 paper, this is equivalent to the "Non-terminal set". In that paper it is also implied that
//! a end-user of the framework would need to provide a Terminal Set, but in my implementation the Terminal set is represented through
//! the outputs/inputs defined in the Non-terminal Grammar.

use crate::{
    node::Node, nonterminal::{NonTerminalGrammar, NonTerminalRule}, possibilities_tables::PossibilityTable, types::{DataType, Dataset, GenerationMethod, Shape, TypeInfo, Variable, VariableDefinitions}, variable::VariableContext
};
use std::any::Any;
use rand::Rng;

#[derive(Debug)]
struct ParseTree {
    id: usize,
    tree: Vec<Node>,
}

impl ParseTree {
    fn empty(id: usize) -> Self {
        ParseTree { id, tree: Vec::new() }
    }

    fn insert_node() {
        unimplemented!();
    }

    fn delete_node() {
        unimplemented!();
    }
    fn evaluate(runtime_variables: &VariableContext) {
        unimplemented!();
    }
    fn mutate(&mut self, nt_grammar: &NonTerminalGrammar) {
        unimplemented!();
    }

    fn generate_random(
        id: usize,
        max_depth: usize,
        required_output_type: TypeInfo,
        nt_grammar: &NonTerminalGrammar,
        variable_definitions: &VariableDefinitions,
        generation_method: GenerationMethod,
    ) -> Self {
        let mut tree = ParseTree::empty(id);
        let mut rng = rand::rng();
        
        // Generate the root node
        let root_idx = tree.generate_node_recursive(
            0, // current depth 
            max_depth,
            required_output_type,
            nt_grammar,
            variable_definitions,
            &mut rng,
            generation_method,
            0, // parent index (root has no parent, will be adjusted)
        );
        
        tree
    }
    
    fn generate_node_recursive(
        &mut self,
        current_depth: usize,
        max_depth: usize,
        required_type: TypeInfo,
        nt_grammar: &NonTerminalGrammar,
        variable_definitions: &VariableDefinitions,
        rng: &mut impl Rng,
        generation_method: GenerationMethod,
        parent_idx: usize,
    ) -> usize {
        let current_idx = self.tree.len();
        
        // Determine if we should create a terminal or non-terminal
        let should_be_terminal = match generation_method {
            GenerationMethod::Full => current_depth >= max_depth - 1,
            GenerationMethod::Grow => {
                if current_depth >= max_depth - 1 {
                    true // Must be terminal at max depth
                } else {
                    // Randomly choose between terminal and non-terminal
                    rng.random_bool(0.3) // 30% chance of terminal at non-leaf levels
                }
            }
        };
        
        if should_be_terminal {
            // Create terminal node
            self.create_terminal_node(required_type, variable_definitions, rng, current_idx, parent_idx)
        } else {
            // Create non-terminal node
            self.create_nonterminal_node(
                current_depth,
                max_depth,
                required_type,
                nt_grammar,
                variable_definitions,
                rng,
                generation_method,
                current_idx,
                parent_idx,
            )
        }
    }
    
    fn create_terminal_node(
        &mut self,
        required_type: TypeInfo,
        variable_definitions: &VariableDefinitions,
        rng: &mut impl Rng,
        current_idx: usize,
        parent_idx: usize,
    ) -> usize {
        // Decide if this should be a variable or constant
        let use_variable = !variable_definitions.variables.is_empty() && rng.random_bool(0.5);
        
        if use_variable {
            // Find variables that match the required type
            let matching_vars: Vec<&Variable> = variable_definitions
                .variables
                .iter()
                .filter(|var| var._type == required_type)
                .collect();
            
            if !matching_vars.is_empty() {
                let chosen_var = matching_vars[rng.random_range(0..matching_vars.len())];
                let placeholder_value = Self::create_placeholder_value(required_type);
                
                let terminal_node = Node {
                    idx: current_idx,
                    _type: crate::node::NodeType::Terminal(required_type),
                    value: placeholder_value,
                    variable_id: Some(chosen_var.name.clone()),
                    left_index: None,
                    right_index: None,
                    parent_index: parent_idx,
                };
                
                self.tree.push(terminal_node);
                return current_idx;
            }
        }
        
        // Create constant terminal
        let random_value = Self::create_random_value(required_type, rng);
        let terminal_node = Node {
            idx: current_idx,
            _type: crate::node::NodeType::Terminal(required_type),
            value: random_value,
            variable_id: None,
            left_index: None,
            right_index: None,
            parent_index: parent_idx,
        };
        
        self.tree.push(terminal_node);
        current_idx
    }
    
    fn create_nonterminal_node(
        &mut self,
        current_depth: usize,
        max_depth: usize,
        required_type: TypeInfo,
        nt_grammar: &NonTerminalGrammar,
        variable_definitions: &VariableDefinitions,
        rng: &mut impl Rng,
        generation_method: GenerationMethod,
        current_idx: usize,
        parent_idx: usize,
    ) -> usize {
        // Get possible input combinations that produce the required output type
        let possible_inputs = nt_grammar.get_all_possible_input_types_with_operations(required_type);
        
        if possible_inputs.is_empty() {
            // No non-terminal rules can produce this type, create a terminal instead
            return self.create_terminal_node(required_type, variable_definitions, rng, current_idx, parent_idx);
        }
        
        // Choose a random input combination
        let (left_type, right_type, operation) = possible_inputs[rng.random_range(0..possible_inputs.len())];
        
        // Create placeholder for the non-terminal node
        let placeholder_value = Self::create_placeholder_value(required_type);
        let nonterminal_node = Node {
            idx: current_idx,
            _type: crate::node::NodeType::NonTerminal(left_type, right_type, operation, required_type),
            value: placeholder_value,
            variable_id: None,
            left_index: None,  // Will be set after creating children
            right_index: None, // Will be set after creating children
            parent_index: parent_idx,
        };
        
        self.tree.push(nonterminal_node);
        
        // Recursively create left and right children
        let left_idx = self.generate_node_recursive(
            current_depth + 1,
            max_depth,
            left_type,
            nt_grammar,
            variable_definitions,
            rng,
            generation_method,
            current_idx,
        );
        
        let right_idx = self.generate_node_recursive(
            current_depth + 1,
            max_depth,
            right_type,
            nt_grammar,
            variable_definitions,
            rng,
            generation_method,
            current_idx,
        );
        
        // Update the non-terminal node with child indices
        self.tree[current_idx].left_index = Some(left_idx);
        self.tree[current_idx].right_index = Some(right_idx);
        
        current_idx
    }
    
    fn create_random_value(type_info: TypeInfo, rng: &mut impl Rng) -> Box<dyn Any> {
        match (type_info.data_type, type_info.shape) {
            (DataType::Integer, Shape::Scalar) => {
                Box::new(rng.random_range(-100..=100i32))
            }
            (DataType::Float, Shape::Scalar) => {
                Box::new(rng.random_range(-100.0..=100.0f64))
            }
            (DataType::Integer, Shape::Vector(size)) => {
                let vec: Vec<i32> = (0..size).map(|_| rng.random_range(-100..=100)).collect();
                Box::new(vec)
            }
            (DataType::Float, Shape::Vector(size)) => {
                let vec: Vec<f64> = (0..size).map(|_| rng.random_range(-100.0..=100.0)).collect();
                Box::new(vec)
            }
            (DataType::Integer, Shape::Matrix(rows, cols)) => {
                let matrix: Vec<Vec<i32>> = (0..rows)
                    .map(|_| (0..cols).map(|_| rng.random_range(-100..=100)).collect())
                    .collect();
                Box::new(matrix)
            }
            (DataType::Float, Shape::Matrix(rows, cols)) => {
                let matrix: Vec<Vec<f64>> = (0..rows)
                    .map(|_| (0..cols).map(|_| rng.random_range(-100.0..=100.0)).collect())
                    .collect();
                Box::new(matrix)
            }
        }
    }
    
    fn create_placeholder_value(type_info: TypeInfo) -> Box<dyn Any> {
        match (type_info.data_type, type_info.shape) {
            (DataType::Integer, Shape::Scalar) => Box::new(0i32),
            (DataType::Float, Shape::Scalar) => Box::new(0.0f64),
            (DataType::Integer, Shape::Vector(size)) => Box::new(vec![0i32; size]),
            (DataType::Float, Shape::Vector(size)) => Box::new(vec![0.0f64; size]),
            (DataType::Integer, Shape::Matrix(rows, cols)) => Box::new(vec![vec![0i32; cols]; rows]),
            (DataType::Float, Shape::Matrix(rows, cols)) => Box::new(vec![vec![0.0f64; cols]; rows]),
        }
    }
}

#[derive(Debug)]
pub struct TreeOrchestrator {
    nt_grammar: NonTerminalGrammar,
    variable_definitions: VariableDefinitions, // Static variable type definitions
    dataset: Dataset, // Training data with inputs and expected outputs
    possibilities_table: PossibilityTable,
    required_output_type: TypeInfo,
    max_depth: usize,
    grow_method: GenerationMethod,
    trees: Vec<ParseTree>,
    tree_scores: Vec<f64>, // Changed to f64 for fitness scores
}

impl TreeOrchestrator {
    pub fn new(nt_grammar: NonTerminalGrammar, variable_definitions: VariableDefinitions, dataset: Dataset, max_depth: usize, required_output_type: TypeInfo) -> Self {
        TreeOrchestrator { 
            nt_grammar,
            variable_definitions,
            dataset,
            required_output_type,
            possibilities_table: PossibilityTable::empty(max_depth),
            max_depth: max_depth,
            grow_method: GenerationMethod::Full, // this is currently the only option supported.
            trees: Vec::new(),
            tree_scores: Vec::new(),
        }
    }

    pub fn generate_empty_trees(&mut self, generation_size: usize) {
        for i in 0..generation_size {
            self.trees.push(ParseTree::empty(i));
        }
    }

    pub fn generate_trees(&mut self, generation_size: usize) {
        for i in 0..generation_size {
            self.trees.push(ParseTree::generate_random(
                i,
                self.max_depth,
                self.required_output_type,
                &self.nt_grammar,
                &self.variable_definitions,
                self.grow_method,
            ));
        }
    }
    
    pub fn get_variable_definitions(&self) -> &VariableDefinitions {
        &self.variable_definitions
    }
    
    pub fn get_dataset(&self) -> &Dataset {
        &self.dataset
    }
    
    // Method to create VariableContext from a specific dataset row for evaluation
    // Note: This creates a reference-based context that borrows from the dataset
    pub fn create_variable_context_for_row(&self, row_index: usize) -> Result<&crate::types::DataRow, String> {
        if row_index >= self.dataset.inputs.len() {
            return Err(format!("Row index {} out of bounds", row_index));
        }
        
        Ok(&self.dataset.inputs[row_index])
    }
    
    // Helper method to get expected output for a row
    pub fn get_expected_output(&self, row_index: usize) -> Result<&Box<dyn Any>, String> {
        if row_index >= self.dataset.expected_outputs.len() {
            return Err(format!("Output index {} out of bounds", row_index));
        }
        
        Ok(&self.dataset.expected_outputs[row_index])
    }

    pub fn construct_possibilities_table(&mut self) {
        unimplemented!()
    }
}
