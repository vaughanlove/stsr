use std::fmt;
use super::terminal::{Terminal, GenericFunction};
//// Types for the tree structure used in STGP


/// A node in the tree. Can be a Function or Terminal. Note that this only allows downward traversal of the tree.
#[derive(Debug, Clone)]
pub enum Node {
    Terminal(Terminal),
    
    /// A function node with children
    Function {
        function: GenericFunction,
        children: Vec<Node>, // Use Vec for variable arity
    },
}



impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Terminal(terminal) => write!(f, "Terminal({})", terminal),
            Node::Function{function, children} => write!(f, "Function({})", function),
        }
    }
}

