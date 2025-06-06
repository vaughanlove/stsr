use crate::types::{ DataType};
use crate::ops::{Operation};
use crate::node::{ Node};
use std::fmt;


impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operation::Add => {
                write!(f, "+")
            },
            Operation::Subtract => {
                write!(f, "-")
            },
            Operation::Divide => {
                write!(f, "/")
            }
            &Operation::Multiply => {
                write!(f, "*")
            }
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Node Index {} \n", self.idx)
        // write!(f, "Value {} \n", self.value)
    }
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataType::Float => {
                write!(f, "FLOAT")
            },
            DataType::Integer => {
                write!(f, "INT")
            }
        }
    }
}

// impl fmt::Display for Shape {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Shape::Scalar => {
//                 write!(f, "SCALAR")
//             }
//         }
//     }
// }

// impl fmt::Display for NodeType {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             NodeType::NonTerminal(operation) => {
//                 write!(f, "NonTerminal({})", operation)
//             },
//             NodeType::Terminal(data_type, shape) => {
//                 write!(f, "Terminal({}, {})", data_type, shape)
//             }
//         }
//     }
// }