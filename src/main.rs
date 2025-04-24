mod types;
mod registries;
use types::terminal::{PrimitiveType, Terminal, TerminalShape, GenericParam};
use types::tree::{Node};


fn main() {
    // Create a vector terminal with float elements and GENNUM1 dimension
    let vector_terminal = Terminal::Variable(
        TerminalShape::Vector(
            PrimitiveType::Float,
            GenericParam::Name("GENNUM1".to_string())
        )
    );
    
    // Print it using debug formatting
    println!("Vector terminal: {}", vector_terminal);
    
    // Create a matrix terminal
    let matrix_terminal = Terminal::Variable(
        TerminalShape::Matrix(
            PrimitiveType::Float,
            GenericParam::Name("GENNUM1".to_string()),
            GenericParam::Name("GENNUM2".to_string())
        )
    );
    
    println!("Matrix terminal: {}", matrix_terminal);
    
    // Create a scalar constant
    let scalar_constant: Terminal = Terminal::Constant(
        TerminalShape::Scalar(PrimitiveType::Integer)
    );

    let node: Node = Node::Terminal(vector_terminal);
    
    println!("Node: {}", node);
}
