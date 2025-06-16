// use stsr::arena::{Arena, GenerationMethod};
use stsr::node::compatible_inputs;
use stsr::nonterminal::NonTerminalGrammar;
use stsr::shape::ShapeGrammar;
use stsr::types::{DataType, Shape, Variable};

fn main() {
    // test_arena_constrution();
    // test_compatible_inputs();
    create_terminal_set();
    test_create_nonterminal_registry();
}


fn test_create_nonterminal_registry() {
    let mut nt_grammar = NonTerminalGrammar::new();

    // the user should be able to very easily define the operations they want to support.
    
    // Example: integer scalar addition
    let add_rule = stsr::nonterminal::NonTerminalRule::scalar_arithmetic(
        DataType::Integer,
        stsr::ops::Operation::Add,
        |a, b| {
            let val_a = a.downcast_ref::<i32>().unwrap();
            let val_b = b.downcast_ref::<i32>().unwrap();
            Box::new(val_a + val_b)
        }
    );
    
    // Example: integer scalar multiplication
    let mult_rule = stsr::nonterminal::NonTerminalRule::scalar_arithmetic(
        DataType::Integer,
        stsr::ops::Operation::Multiply,
        |a, b| {
            let val_a = a.downcast_ref::<i32>().unwrap();
            let val_b = b.downcast_ref::<i32>().unwrap();
            Box::new(val_a * val_b)
        }
    );
    
    nt_grammar.add_rule(add_rule);
    nt_grammar.add_rule(mult_rule);
    
    println!("NonTerminal grammar has {} rules", nt_grammar.rules.len());
}

fn test_compatible_inputs() {
    let op = stsr::ops::Operation::Divide;
    let output = stsr::types::TypeInfo { shape: Shape::Vector(2), _type: DataType::Integer };
    let comp = compatible_inputs(op, output);

    println!("{:?}", comp);
}

fn create_terminal_set() {
    // let user define types.
    let type_one = stsr::types::TypeInfo { shape: Shape::Vector(2), _type: DataType::Integer };
    let type_two = stsr::types::TypeInfo { shape: Shape::Vector(1), _type: DataType::Integer };
    let type_three = stsr::types::TypeInfo { shape: Shape::Scalar, _type: DataType::Integer };

    let terminal_set = vec![type_one, type_two, type_three];
    // let user define opertaions + their interactions.
    // we then handle the rest.
    
    // let nonterminal_set = vec![];

    println!("{:?}", terminal_set);
}

fn test_arena_constrution() {
    println!("=== Testing Arena Construction ===");

    let var_one = Variable {name: "x".to_string(), data_type: DataType::Integer, shape: Shape::Scalar};
    let var_two = Variable {name: "y".to_string(), data_type: DataType::Integer, shape: Shape::Scalar};

    let required_vars = vec![var_one, var_two];

    let grammar = ShapeGrammar::new(3);

    // let mut arena = Arena::init_with_method(GenerationMethod::Full, 3, DataType::Integer, Shape::Scalar, required_vars, grammar);
    
    // println!("{:?}", arena);
    // println!("{:?}", grammar.generate_all_shapes());

    // let compatible = grammar.compatible_shapes(&Shape::Matrix(2,3));
    // println!("{:?}", compatible);
}

