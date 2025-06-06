use stsr::arena::{Arena, GenerationMethod};
use stsr::node::{Node, NodeType};
use stsr::types::{DataType, Shape, Variable};
use stsr::ops::Operation;
use stsr::variable::{VariableContext};

fn main() {
    test_arena_constrution();
}

fn test_arena_constrution() {
    println!("=== Testing Arena Construction ===");

    let var_one = Variable {name: "x".to_string(), data_type: DataType::Integer, shape: Shape::Scalar};
    let var_two = Variable {name: "y".to_string(), data_type: DataType::Integer, shape: Shape::Scalar};

    let required_vars = vec![var_one, var_two];
    let mut arena = Arena::init_with_method(GenerationMethod::Full, 15, DataType::Integer, Shape::Scalar, required_vars);
    
    println!("{:?}", arena);
    
    

}

