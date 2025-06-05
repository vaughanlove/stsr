use stsr::node::{Node, NodeType};
use stsr::types::{DataType, Shape};
use stsr::ops::Operation;
use stsr::variable::{Variable, VariableContext};

fn main() {
    test_constants();
    test_variables();
}

fn test_constants() {
    println!("=== Testing Constants ===");
    // Create a simple tree: 5 + 3
    let mut arena = Vec::new();
    
    // Terminal node: 5 (index 0)
    let left_terminal = Node::new_terminal(0, None, 5i64, 0, 0, 2);
    arena.push(left_terminal);
    
    // Terminal node: 3 (index 1)  
    let right_terminal = Node::new_terminal(1, None, 3i64, 0, 0, 2);
    arena.push(right_terminal);
    
    // Non-terminal node: Add (index 2, root)
    let root = Node::new_non_terminal(2, None, Operation::Add, "add".to_string(), 0, 1, 2);
    arena.push(root);
    
    // Evaluate the tree
    let result = arena[2].evaluate(&arena);
    
    match result {
        Ok(value) => {
            if let Some(result_val) = value.downcast_ref::<i64>() {
                println!("5 + 3 = {}", result_val);
            } else {
                println!("Failed to extract result value");
            }
        }
        Err(e) => println!("Evaluation error: {}", e),
    }
}

fn test_variables() {
    println!("\n=== Testing Variables ===");
    // Create a tree: x + y
    let mut arena = Vec::new();
    
    // Variable terminal: x (index 0)
    let x_terminal = Node {
        idx: 0,
        variable_id: Some("x".to_string()),
        _type: NodeType::Terminal(DataType::Integer, Shape::Scalar),
        value: Box::new(0i64), // placeholder
        left: None,
        right: None,
        parent: 2,
    };
    arena.push(x_terminal);
    
    // Variable terminal: y (index 1)  
    let y_terminal = Node {
        idx: 1,
        variable_id: Some("y".to_string()),
        _type: NodeType::Terminal(DataType::Integer, Shape::Scalar),
        value: Box::new(0i64), // placeholder
        left: None,
        right: None,
        parent: 2,
    };
    arena.push(y_terminal);
    
    // Non-terminal node: Add (index 2, root)
    let root = Node::new_non_terminal(2, None, Operation::Add, "add".to_string(), 0, 1, 2);
    arena.push(root);
    
    // Create variable context
    let mut context = VariableContext::new();
    context.add_variable(Variable::new("x".to_string(), DataType::Integer, Shape::Scalar, 10i64));
    context.add_variable(Variable::new("y".to_string(), DataType::Integer, Shape::Scalar, 7i64));
    
    // Evaluate with context
    let result = arena[2].evaluate_with_context(&arena, &context);
    
    match result {
        Ok(value) => {
            if let Some(result_val) = value.downcast_ref::<i64>() {
                println!("x + y = 10 + 7 = {}", result_val);
            } else {
                println!("Failed to extract result value");
            }
        }
        Err(e) => println!("Evaluation error: {}", e),
    }
    
    // Test changing variable values
    context.set_variable_value("x", 20i64).unwrap();
    context.set_variable_value("y", 15i64).unwrap();
    
    let result2 = arena[2].evaluate_with_context(&arena, &context);
    match result2 {
        Ok(value) => {
            if let Some(result_val) = value.downcast_ref::<i64>() {
                println!("x + y = 20 + 15 = {}", result_val);
            } else {
                println!("Failed to extract result value");
            }
        }
        Err(e) => println!("Evaluation error: {}", e),
    }
}
