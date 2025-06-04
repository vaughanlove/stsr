use stsr::arena::Arena;
use stsr::node::{Node, NodeType};
use stsr::types::{DataType, Shape};
fn main() {
    // Create a vector terminal with float elements and GENNUM1 dimension    
    println!("Hello world!");

    let arena = Arena::init();

    let nt = NodeType::Terminal(DataType::Integer, Shape::Scalar);

    println!("{:}", &nt);

    let node = Node {
        idx: 0,
        _type: nt,
        value: Box::new(123),
        left: 0,
        right: 0,
        parent: 0
    };

    println!("{:}", &node)

}
