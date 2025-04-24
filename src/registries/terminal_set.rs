use crate::types;
use types::terminal::*;
use std::vec::Vec;

pub struct TerminalRegistry {
    accepted_terminals: Vec<Terminal>
}

impl Default for TerminalRegistry {
    fn default() -> TerminalRegistry{
        TerminalRegistry {
            accepted_terminals: Vec::new()
        }
    }
}

// impl TerminalRegistry {
//     fn add(&self, t: Terminal) -> Vec<Terminal> {
//         &mut self.accepted_terminals.push(t)
//     }
// }