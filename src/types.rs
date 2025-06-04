use std::any::Any;
use rand::Rng;


/// Base Types in the system. 
/// 
/// TODO: The developer needs a way to specify a subset of these for their genetic program. 
#[derive(Copy, Clone)]
pub enum DataType {
    Integer,
    Float,
}

/// The shape that a terminal can take. 
#[derive(Copy, Clone)]
pub enum Shape {
    Scalar,
    Vector(usize),
    Matrix(usize, usize),
}

// type GenericFunction = dyn Fn(&DataType, &DataType) -> DataType;


// trait Executable {
//     // fn get_value(&self) -> Self;
//     // fn get_type(&self) -> Self;
// }







