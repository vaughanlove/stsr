//! Tree building utilities and relating data structures.
//!
//! This module provides functionality to create trees given a Non-terminal Grammar. It provides the ability to build possibility tables
//! from this grammar. Note that in Montana's 1995 paper, this is equivalent to the "Non-terminal set". In that paper it is also implied that
//! a end-user of the framework would need to provide a Terminal Set, but in my implementation the Terminal set is represented through
//! the outputs/inputs defined in the Non-terminal Grammar.

use crate::{
    node::Node,
    node::NodeType,
    nonterminal::{NonTerminalGrammar, NonTerminalRule},
    possibilities_tables::PossibilityTable,
    types::{
        DataRow, DataType, Dataset, EvalInput, GenerationMethod, Shape, TypeInfo, Variable,
        VariableDefinitions,
    },
    variable::VariableContext,
};
use rand::Rng;
use std::{any::Any, rc::Rc};

#[derive(Debug)]
pub struct ParseTree {
    pub id: usize,
    pub fitness: f64,
    pub tree: Vec<Node>,
}

impl ParseTree {
    fn empty(id: usize) -> Self {
        ParseTree {
            id,
            fitness: 0.0,
            tree: Vec::new(),
        }
    }

    fn evaluate_fitness(&mut self, dataset: &Dataset, grammar: &NonTerminalGrammar) {
        let mut fitness: f64 = 0.0;

        for eval_input in dataset.iter() {
            self.evaluate(&eval_input, grammar);

            // Extract target from EvalInput
            let target = match eval_input {
                EvalInput::Data(_, target_rc) => target_rc,
            };

            // Get prediction from tree
            let prediction = match &self.tree[0]._type {
                NodeType::NonTerminal(_,_,_,TypeInfo {
                    shape: _,
                    data_type: DataType::Float,
                }) => {
                    let value = self.tree[0].value.downcast_ref::<f64>().unwrap();
                    *value
                }
                NodeType::NonTerminal(_,_,_,TypeInfo {
                    shape: _,
                    data_type: DataType::Integer,
                }) => {
                    let value = self.tree[0].value.downcast_ref::<i32>().unwrap();
                    *value as f64 // Convert to f64 for consistent math
                }
                _ => {
                    panic!(
                        "Cannot evaluate fitness: node at index 0 has invalid type {:?}",
                        self.tree[0]._type
                    );
                }
            };
            
            // Downcast target and compute loss
            let loss = if let Some(target_f64) = target.downcast_ref::<f64>() {
                (prediction - target_f64).abs() // L1 loss
            } else if let Some(target_i32) = target.downcast_ref::<i32>() {
                (prediction - (*target_i32 as f64)).abs() // L1 loss
            } else {
                // Handle unsupported target type
                eprintln!("Unsupported target type");
                continue;
            };

            fitness += loss;
        }

        self.fitness = fitness;
        println!("TREE HAD FITNESS {:?}", &self.fitness);
    }

    fn evaluate(&mut self, data: &EvalInput, grammar: &NonTerminalGrammar) {
        match data {
            EvalInput::Data(vars, _) => {
                for i in (0..self.tree.len()).rev() {
                    if !self.tree[i].is_leaf_node() {
                        self.evaluate_node_at_index(i, vars, grammar);
                    }
                }
            }
        }
    }

    fn evaluate_node_at_index(&mut self, idx: usize, vars: &DataRow, grammar: &NonTerminalGrammar) {
        match (self.tree[idx].left_index, self.tree[idx].right_index) {
            // NonTerminal node - evaluate using child values
            (Some(left_idx), Some(right_idx)) => {
                // Find matching rule in grammar
                let rule = self.find_matching_rule_for_node(idx, grammar).unwrap();

                // Execute the operation with child values
                let result = rule.execute(
                    self.tree[left_idx].value.as_ref(),
                    self.tree[right_idx].value.as_ref(),
                );
                // Store result in current node
                self.tree[idx].value = result;
            }
            // Terminal node - handle variable lookup if needed
            (None, None) => {
                if let Some(variable_id) = &self.tree[idx].variable_id {
                    let var_value = vars
                        .values
                        .get(variable_id)
                        .expect(&format!("Variable '{}' not found in data row", variable_id));
                    // Convert Rc<dyn Any> to Box<dyn Any>
                    // We clone the Rc (cheap) and then convert it to Box
                    self.tree[idx].value = Box::new(var_value.clone());
                }
                // If no variable_id, value is already set (constant terminal)
            }
            _ => panic!("Invalid node configuration: partial children"),
        }
    }

    fn find_matching_rule_for_node<'a>(
        &self,
        idx: usize,
        grammar: &'a NonTerminalGrammar,
    ) -> Result<&'a NonTerminalRule, String> {
        let (input1_type, input2_type, operation, output_type) = match self.tree[idx]._type {
            crate::node::NodeType::NonTerminal(i1, i2, op, o) => (i1, i2, op, o),
            _ => return Err("Expected NonTerminal node type".to_string()),
        };

        grammar
            .rules
            .iter()
            .find(|rule| {
                rule.operation == operation
                    && rule.input_one_type == input1_type
                    && rule.input_two_type == input2_type
                    && rule.output == output_type
            })
            .ok_or_else(|| format!("No matching rule found for operation {:?}", operation))
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
        possibilities_table: &PossibilityTable,
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
            possibilities_table,
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
        possibilities_table: &PossibilityTable,
    ) -> usize {
        let current_idx = self.tree.len();

        // Check if the required type is possible at this depth using the PossibilityTable
        if !possibilities_table.can_produce_type_at_depth(current_depth, required_type) {
            // If the type is not possible at this depth, we have a constraint violation
            // This should not happen with a properly constructed possibilities table,
            // but we'll handle it gracefully by creating a terminal

            // todo, make this panic?
            return self.create_terminal_node(
                required_type,
                variable_definitions,
                rng,
                current_idx,
                parent_idx,
            );
        }

        // Determine if we should create a terminal or non-terminal
        let should_be_terminal = match generation_method {
            GenerationMethod::Full => current_depth >= max_depth - 1,
            GenerationMethod::Grow => {
                if current_depth >= max_depth - 1 {
                    true // Must be terminal at max depth
                } else {
                    // Randomly choose between terminal and non-terminal
                    // But only if both are possible according to the possibilities table
                    let can_be_nonterminal = !nt_grammar
                        .get_all_possible_input_types_with_operations(required_type)
                        .is_empty();
                    let can_be_terminal = variable_definitions
                        .variables
                        .iter()
                        .any(|var| var._type == required_type);

                    if can_be_nonterminal && can_be_terminal {
                        rng.random_bool(0.3) // 30% chance of terminal at non-leaf levels
                    } else if can_be_terminal {
                        true // Only terminal is possible
                    } else {
                        false // Only non-terminal is possible
                    }
                }
            }
        };

        if should_be_terminal {
            // Create terminal node
            self.create_terminal_node(
                required_type,
                variable_definitions,
                rng,
                current_idx,
                parent_idx,
            )
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
                possibilities_table,
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
        possibilities_table: &PossibilityTable,
    ) -> usize {
        // Get possible input combinations that produce the required output type
        let all_possible_inputs =
            nt_grammar.get_all_possible_input_types_with_operations(required_type);

        if all_possible_inputs.is_empty() {
            // No non-terminal rules can produce this type, create a terminal instead
            return self.create_terminal_node(
                required_type,
                variable_definitions,
                rng,
                current_idx,
                parent_idx,
            );
        }

        // Filter input combinations based on what's possible at the next depth
        let next_depth = current_depth + 1;
        let valid_inputs: Vec<_> = all_possible_inputs
            .into_iter()
            .filter(|(left_type, right_type, _)| {
                // Both input types must be possible at the next depth
                possibilities_table.can_produce_type_at_depth(next_depth, *left_type)
                    && possibilities_table.can_produce_type_at_depth(next_depth, *right_type)
            })
            .collect();

        if valid_inputs.is_empty() {
            // No valid input combinations according to possibilities table, create terminal
            return self.create_terminal_node(
                required_type,
                variable_definitions,
                rng,
                current_idx,
                parent_idx,
            );
        }

        // Choose a random valid input combination
        let (left_type, right_type, operation) =
            valid_inputs[rng.random_range(0..valid_inputs.len())];

        // Create placeholder for the non-terminal node
        let placeholder_value = Self::create_placeholder_value(required_type);
        let nonterminal_node = Node {
            idx: current_idx,
            _type: crate::node::NodeType::NonTerminal(
                left_type,
                right_type,
                operation,
                required_type,
            ),
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
            possibilities_table,
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
            possibilities_table,
        );

        // Update the non-terminal node with child indices
        self.tree[current_idx].left_index = Some(left_idx);
        self.tree[current_idx].right_index = Some(right_idx);

        current_idx
    }

    fn create_random_value(type_info: TypeInfo, rng: &mut impl Rng) -> Box<dyn Any> {
        match (type_info.data_type, type_info.shape) {
            (DataType::Integer, Shape::Scalar) => Box::new(rng.random_range(-100..=100i32)),
            (DataType::Float, Shape::Scalar) => Box::new(rng.random_range(-100.0..=100.0f64)),
            (DataType::Integer, Shape::Vector(size)) => {
                let vec: Vec<i32> = (0..size).map(|_| rng.random_range(-100..=100)).collect();
                Box::new(vec)
            }
            (DataType::Float, Shape::Vector(size)) => {
                let vec: Vec<f64> = (0..size)
                    .map(|_| rng.random_range(-100.0..=100.0))
                    .collect();
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
                    .map(|_| {
                        (0..cols)
                            .map(|_| rng.random_range(-100.0..=100.0))
                            .collect()
                    })
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
            (DataType::Integer, Shape::Matrix(rows, cols)) => {
                Box::new(vec![vec![0i32; cols]; rows])
            }
            (DataType::Float, Shape::Matrix(rows, cols)) => {
                Box::new(vec![vec![0.0f64; cols]; rows])
            }
        }
    }
}

#[derive(Debug)]
pub struct TreeOrchestrator {
    nt_grammar: NonTerminalGrammar,
    variable_definitions: VariableDefinitions, // Static variable type definitions
    dataset: Dataset,                          // Training data with inputs and expected outputs
    possibilities_table: PossibilityTable,
    required_output_type: TypeInfo,
    max_depth: usize,
    grow_method: GenerationMethod,
    pub trees: Vec<ParseTree>,
    tree_scores: Vec<f64>, // Changed to f64 for fitness scores
}

impl TreeOrchestrator {
    pub fn new(
        nt_grammar: NonTerminalGrammar,
        variable_definitions: VariableDefinitions,
        dataset: Dataset,
        max_depth: usize,
        required_output_type: TypeInfo,
    ) -> Self {
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
        // Ensure possibilities table is constructed before generation
        if !self.possibilities_table.is_valid_for_generation() {
            self.construct_possibilities_table();
        }

        for i in 0..generation_size {
            self.trees.push(ParseTree::generate_random(
                i,
                self.max_depth,
                self.required_output_type,
                &self.nt_grammar,
                &self.variable_definitions,
                self.grow_method,
                &self.possibilities_table,
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
    pub fn create_variable_context_for_row(
        &self,
        row_index: usize,
    ) -> Result<&crate::types::DataRow, String> {
        if row_index >= self.dataset.features.len() {
            return Err(format!("Row index {} out of bounds", row_index));
        }

        Ok(&self.dataset.features[row_index])
    }

    // Helper method to get expected output for a row
    pub fn get_expected_output(&self, row_index: usize) -> Result<&Rc<dyn Any>, String> {
        if row_index >= self.dataset.targets.len() {
            return Err(format!("Output index {} out of bounds", row_index));
        }

        Ok(&self.dataset.targets[row_index])
    }

    // now for the time being this is operating under the understanding that runtime variables is a hashmap in memory that gets updated EVERYTIME evaluate wants to get called.
    // I don't know if this should happen in evaluate_trees or elsewhere, but that can be figured out.
    // perhaps Dataset should have a method that allows it to decompose into runtime variables? that seems cleanish.
    // also keeping in mind that we want this to be super parallel. t
    // these will own their values, I believe
    pub fn evaluate_trees<'a>(&mut self, data: &'a EvalInput) {
        for tree in &mut self.trees {
            tree.evaluate(data, &self.nt_grammar);
        }
    }

    /// Evaluates the trees fitness values against the internally stored Dataset.
    /// Stores their fitness value in each ParseTree.
    pub fn evaluate_fitness(&mut self) {
        for tree in self.trees.iter_mut() {
            tree.evaluate_fitness(&self.dataset, &self.nt_grammar);
        }
    }

    pub fn construct_possibilities_table(&mut self) {
        self.possibilities_table = PossibilityTable::new(
            &self.nt_grammar,
            &self.variable_definitions,
            self.required_output_type,
            self.max_depth,
        );
    }

    pub fn get_possibilities_table(&self) -> &PossibilityTable {
        &self.possibilities_table
    }
}
