use crate::types::{DataType, Shape, TypeInfo};

/// Grammar uniting the type system for type and shape

#[derive(Debug)]
struct Grammar {
    shapes: Vec<Shape>,
    types: Vec<DataType>
}

// impl Grammar {
//     fn get_shapes() {

//     }

//     fn get_types() {

//     }

//     /// Function to compute a vector of TypeInfo that contains all terminals that the  
//     fn get_compatible_types(input: TypeInfo) -> Vec<TypeInfo> {
//         match (input.shape, input._type) {
//         (Shape::Scalar, DataType::Integer) => {
//             // Return compatible types for scalar int32
//             vec![/* compatible TypeInfo instances */]
//         }
//         (Shape::Vector(size), DataType::Float) => {
//             // Return compatible types for float32 vector
//             vec![/* compatible TypeInfo instances */]
//         }
//         // Add more patterns as needed
//         _ => Vec::new(),
//     }

//         return Vec::new()
//     }
// }


// tree instantiation issue,
// how do current implementations handle this need to absolutely include variables?