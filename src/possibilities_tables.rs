/// Possibility tables outlined in Montana's paper on page 10.
/// Implemented in rust HashSet to simplify logic.

use std::collections::HashSet;
use std::vec::Vec;
use crate::types::TypeInfo;

#[derive(Debug)]
pub struct PossibilityTable {
    possibilities: Vec<HashSet<TypeInfo>>
}

impl PossibilityTable {
    pub fn empty(max_depth: usize) -> Self {        
        PossibilityTable { 
            possibilities: Vec::with_capacity(max_depth)
        }
    }
}

