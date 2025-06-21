/// Possibility tables outlined in Montana's paper on page 10.
/// Each row represents the possible types at a specific depth of the tree.
/// Derived from nonterminal rules to ensure type safety during tree generation.

use std::collections::HashSet;
use std::vec::Vec;
use crate::types::{TypeInfo, VariableDefinitions};
use crate::nonterminal::NonTerminalGrammar;

#[derive(Debug)]
pub struct PossibilityTable {
    possibilities: Vec<HashSet<TypeInfo>>,
    max_depth: usize,
}

impl PossibilityTable {
    pub fn empty(max_depth: usize) -> Self {        
        PossibilityTable { 
            possibilities: Vec::with_capacity(max_depth),
            max_depth,
        }
    }

    pub fn new(
        grammar: &NonTerminalGrammar,
        variables: &VariableDefinitions,
        target_type: TypeInfo,
        max_depth: usize,
    ) -> Self {
        let mut table = Self::empty(max_depth);
        table.build_from_grammar(grammar, variables, target_type);
        table
    }

    pub fn build_from_grammar(
        &mut self,
        grammar: &NonTerminalGrammar,
        variables: &VariableDefinitions,
        target_type: TypeInfo,
    ) {
        self.possibilities.clear();
        self.possibilities.resize(self.max_depth, HashSet::new());

        // Depth 0 (root) must produce the target type
        self.possibilities[0].insert(target_type);
        // Build possibilities for each depth level
        for depth in 0..self.max_depth - 1 {
            let current_types = self.possibilities[depth].clone();
            println!("CURRENT TYPES: {:?} DEPTH [{:?}]", depth, current_types);

            for current_type in current_types {
                // Get all possible input combinations that can produce this type
                let input_combinations = grammar.get_all_possible_input_types_with_operations(current_type);
                println!("INPUT COMBINATIONS for depth {:?}: {:?}", &depth, input_combinations);
                for (input1_type, input2_type, _) in input_combinations {
                    // Add both input types as possibilities for the next depth
                    self.possibilities[depth + 1].insert(input1_type);
                    self.possibilities[depth + 1].insert(input2_type);
                }
            }
        }

        // At maximum depth, we can only have terminals (variables)
        let terminal_types: HashSet<TypeInfo> = variables.variables
            .iter()
            .map(|var| var._type)
            .collect();
        
        if let Some(last_depth) = self.possibilities.last_mut() {
            last_depth.extend(terminal_types);
        }
    }

    pub fn get_possible_types_at_depth(&self, depth: usize) -> Option<&HashSet<TypeInfo>> {
        self.possibilities.get(depth)
    }

    pub fn can_produce_type_at_depth(&self, depth: usize, type_info: TypeInfo) -> bool {
        self.possibilities
            .get(depth)
            .map_or(false, |types| types.contains(&type_info))
    }

    pub fn get_max_depth(&self) -> usize {
        self.max_depth
    }

    pub fn is_valid_for_generation(&self) -> bool {
        // Check if table has been built and all depth levels have at least one possible type
        !self.possibilities.is_empty() && 
        self.possibilities.iter().all(|types| !types.is_empty())
    }
}

