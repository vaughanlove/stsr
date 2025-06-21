// use stsr::arena::{Arena, GenerationMethod};
use stsr::nonterminal::NonTerminalGrammar;
use stsr::types::{DataType, Dataset, EvalInput, GenerationMethod, Shape, TypeInfo, Variable, VariableDefinitions};
use stsr::ops::Operation;
use stsr::tree_builder::TreeOrchestrator;
use std::any::Any;
use std::collections::HashMap;
use std::rc::Rc;

fn main() {
    // test_arena_constrution();
    // test_compatible_inputs();
    // create_terminal_set();
    test_create_nonterminal_registry();
    test_random_tree_generation();
}


fn test_create_nonterminal_registry() {
    let mut nt_grammar = NonTerminalGrammar::new();

    // the user should be able to very easily define the operations they want to support.

    let ScalarFloat = TypeInfo { 
            shape: Shape::Scalar, 
            data_type: DataType::Float 
        };

    let ScalarInt = TypeInfo { 
            shape: Shape::Scalar, 
            data_type: DataType::Integer 
        };

    // I think it will be on the user to define their operations and how interactions should work.
    let float_add_rule = stsr::nonterminal::NonTerminalRule::new(
        ScalarFloat,
        ScalarInt,
        Operation::Add,
         ScalarInt,
        |a, b| {
            // downcast the float to a int, losing precision. 
            let val_a = a.downcast_ref::<i32>().unwrap();
            let val_b = b.downcast_ref::<i32>().unwrap();
            Box::new(val_a + val_b)
        }
    );

    // Example: integer scalar addition
    let add_rule = stsr::nonterminal::NonTerminalRule::scalar_arithmetic(
        DataType::Integer,
       Operation::Add,
        |a, b| {
            let val_a = a.downcast_ref::<i32>().unwrap();
            let val_b = b.downcast_ref::<i32>().unwrap();
            Box::new(val_a + val_b)
        }
    );
    
    // Example: integer scalar multiplication
    let mult_rule = stsr::nonterminal::NonTerminalRule::scalar_arithmetic(
        DataType::Integer,
        Operation::Multiply,
        |a, b| {
            let val_a = a.downcast_ref::<i32>().unwrap();
            let val_b = b.downcast_ref::<i32>().unwrap();
            Box::new(val_a * val_b)
        }
    );
    
    nt_grammar.add_rule(add_rule);
    nt_grammar.add_rule(mult_rule);
    nt_grammar.add_rule(float_add_rule);

    println!("NonTerminal grammar has {} rules", nt_grammar.rules.len());

    let dummy_type = TypeInfo { data_type: DataType::Integer, shape: Shape::Scalar };

    let dummy_types: Vec<(TypeInfo, TypeInfo, stsr::ops::Operation)> = nt_grammar.get_all_possible_input_types_with_operations(dummy_type);

    println!("dummy types: {:?}", &dummy_types);


}

fn test_random_tree_generation() {
    println!("\n=== Testing Random Tree Generation ===");
    
    // Create a non-terminal grammar
    let mut nt_grammar = NonTerminalGrammar::new();
    
    let scalar_int = TypeInfo { 
        shape: Shape::Scalar, 
        data_type: DataType::Integer 
    };
    
    let scalar_float = TypeInfo { 
        shape: Shape::Scalar, 
        data_type: DataType::Float 
    };
    
    // Add homogeneous integer operations
    let int_add_rule = stsr::nonterminal::NonTerminalRule::new(
        scalar_int, scalar_int, Operation::Add, scalar_int,
        |a, b| {
            let val_a = a.downcast_ref::<i32>().unwrap();
            let val_b = b.downcast_ref::<i32>().unwrap();
            Box::new(val_a + val_b)
        }
    );
    
    // Add homogeneous float operations
    let float_mult_rule = stsr::nonterminal::NonTerminalRule::new(
        scalar_float, scalar_float, Operation::Multiply, scalar_float,
        |a, b| {
            let val_a = a.downcast_ref::<f64>().unwrap();
            let val_b = b.downcast_ref::<f64>().unwrap();
            Box::new(val_a * val_b)
        }
    );
    
    // Add mixed-type operations: Float + Integer -> Integer (with conversion)
    let float_int_to_int = stsr::nonterminal::NonTerminalRule::new(
        scalar_float, scalar_int, Operation::Add, scalar_int,
        |a, b| {
            let val_a = a.downcast_ref::<f64>().unwrap();
            let val_b = b.downcast_ref::<i32>().unwrap();
            Box::new((*val_a as i32) + val_b) // Convert float to int, then add
        }
    );
    
    // Add mixed-type operations: Integer * Float -> Float (with promotion)
    let int_float_to_float = stsr::nonterminal::NonTerminalRule::new(
        scalar_int, scalar_float, Operation::Multiply, scalar_float,
        |a, b| {
            let val_a = a.downcast_ref::<i32>().unwrap();
            let val_b = b.downcast_ref::<f64>().unwrap();
            Box::new((*val_a as f64) * val_b) // Promote int to float, then multiply
        }
    );
    
    // Add type conversion: Integer -> Float
    let int_to_float_rule = stsr::nonterminal::NonTerminalRule::new(
        scalar_int, scalar_int, Operation::Add, scalar_float, // Using add as a "convert" operation
        |a, b| {
            let val_a = a.downcast_ref::<i32>().unwrap();
            let val_b = a.downcast_ref::<i32>().unwrap(); // Ignore second input for conversion
            Box::new(*val_a as f64) // Convert to float
        }
    );
    
    nt_grammar.add_rule(int_add_rule);
    nt_grammar.add_rule(float_mult_rule);
    nt_grammar.add_rule(float_int_to_int);
    nt_grammar.add_rule(int_float_to_float);
    nt_grammar.add_rule(int_to_float_rule);
    
    // Create variable definitions with mixed types
    let variables = vec![
        Variable { name: "x".to_string(), _type: scalar_int },
        Variable { name: "y".to_string(), _type: scalar_int },
        Variable { name: "z".to_string(), _type: scalar_float },
        Variable { name: "w".to_string(), _type: scalar_float },
    ];
    let variable_definitions = VariableDefinitions::new(variables);
    
    // Create a simple dataset with mixed types
    let mut input_values = HashMap::new();
    input_values.insert("x".to_string(), Box::new(5i32) as Box<dyn Any>);
    input_values.insert("y".to_string(), Box::new(3i32) as Box<dyn Any>);
    input_values.insert("z".to_string(), Box::new(2.5f64) as Box<dyn Any>);
    input_values.insert("w".to_string(), Box::new(1.5f64) as Box<dyn Any>);
    
    let data_row = stsr::types::DataRow::from_map(&variable_definitions, input_values).unwrap();
    let dataset = Dataset::new(
        vec![data_row],
        vec![Box::new(8.0f64) as Box<dyn Any>] // Float output target
    ).unwrap();
    
    // Create tree orchestrator targeting Float output to test mixed-type propagation
    let mut orchestrator = TreeOrchestrator::new(
        nt_grammar,
        variable_definitions,
        dataset,
        4, // max_depth (increased to allow more complex trees)
        scalar_float, // required_output_type - now targeting Float to test mixed types
    );
    
    // Generate a single random tree
    orchestrator.generate_trees(1);
    
    println!("Generated tree orchestrator with {} trees", orchestrator.trees.len());
    
    // Print the tree structure (we'll need to add a display method)
    // if let Some(tree) = orchestrator.trees.first() {
    //     println!("Tree structure:");
    //     print_tree_structure(tree);
    // }

    let variables_two = vec![
        Variable { name: "x".to_string(), _type: scalar_int },
        Variable { name: "y".to_string(), _type: scalar_int },
        // Variable { name: "z".to_string(), _type: scalar_float },
        // Variable { name: "w".to_string(), _type: scalar_float },
    ];

    let variable_definitions_two = VariableDefinitions::new(variables_two);

    // Create a simple dataset with mixed types
    let mut input_values_two = HashMap::new();
    input_values_two.insert("x".to_string(), Box::new(5i32) as Box<dyn Any>);
    input_values_two.insert("y".to_string(), Box::new(3i32) as Box<dyn Any>);
    // input_values_two.insert("z".to_string(), Box::new(2.5f64) as Box<dyn Any>);
    // input_values_two.insert("w".to_string(), Box::new(1.5f64) as Box<dyn Any>);

    let rdata = stsr::types::DataRow::from_map(&variable_definitions_two, input_values_two).unwrap();

    let tval = Rc::new(8.0f64) as Rc<dyn Any>;
    let data = EvalInput::Data(&rdata, &tval);

    orchestrator.evaluate_trees(&data);
}

fn print_tree_structure(tree: &stsr::tree_builder::ParseTree) {
    println!("Tree has {} nodes", tree.tree.len());
    
    for (i, node) in tree.tree.iter().enumerate() {
        println!("Node {}: {:?}", i, node._type);
        if let Some(var_id) = &node.variable_id {
            println!("  Variable: {}", var_id);
        }
        if let Some(left) = node.left_index {
            println!("  Left child: {}", left);
        }
        if let Some(right) = node.right_index {
            println!("  Right child: {}", right);
        }
        println!("  Parent: {}", node.parent_index);
    }
}

