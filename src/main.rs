// use stsr::arena::{Arena, GenerationMethod};
use stsr::nonterminal::NonTerminalGrammar;
use stsr::types::{DataType, Shape, TypeInfo};
use stsr::ops::Operation;

fn main() {
    // test_arena_constrution();
    // test_compatible_inputs();
    // create_terminal_set();
    test_create_nonterminal_registry();
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

