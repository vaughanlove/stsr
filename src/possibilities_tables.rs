/// Possibility tables outlined in Montana's paper on page 10.
/// Implemented in rust HashSet to simplify logic.

use std::collections::HashSet;
use std::vec::Vec;
use crate::types::TypeInfo;

struct PossibilityTable {
    possibilities: Vec<HashSet<TypeInfo>>
}

impl PossibilityTable {
    fn init(terminal_set: Vec<TypeInfo>, max_depth: usize) -> Self {
        for i in 1..max_depth {
            
        }
        PossibilityTable { 
            possibilities: Vec::new() 
        }
    }
}

