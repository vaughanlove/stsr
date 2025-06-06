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

pub fn get_all_shapes_below_dimension(dim: usize) -> Vec<Shape> {
    let mut shapes: Vec<Shape> = Vec::new();
    for i in 1..dim {
        for j in 1..dim {
            match (i, j) {
                (1, 1) => shapes.push(Shape::Scalar),
                (1, 2) => shapes.push(Shape::Vector(2)),
                (1, 3) => shapes.push(Shape::Vector(3)),
                (2, 1) => (), // Skip this case - duplicate shape to 1,2
                (2, 2) => shapes.push(Shape::Matrix(2, 2)),
                (2, 3) => shapes.push(Shape::Matrix(2, 3)),
                (3, 2) => shapes.push(Shape::Matrix(3, 2)),
                (3, 3) => shapes.push(Shape::Matrix(3,3)),
                _ => unimplemented!("We don't currently support dimensions above 3.")
            }
        }
    }
    shapes
}

// Struct that will be public facing for developers to define their own variables according to their datasets.
#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub data_type: DataType,
    pub shape: Shape,
}