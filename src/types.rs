/// Base Types in the system. 
/// 
/// TODO: The developer needs a way to specify a subset of these for their genetic program. 
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DataType {
    Integer,
    Float,
}

/// The shape that a terminal can take. 
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Shape {
    Scalar,
    Vector(usize),
    Matrix(usize, usize),
}

// Struct that will be public facing for developers to define their own variables according to their datasets.
#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub data_type: DataType,
    pub shape: Shape,
}