// use crate::types::Shape;

// #[derive(Debug, Clone)]
// pub struct ShapeGrammar {
//     max_dim: usize,
// }

// impl ShapeGrammar {
//     pub fn new(max_dim: usize) -> Self {
//         Self { max_dim }
//     }
    
//     pub fn generate_all_shapes(&self) -> Vec<Shape> {
//         let mut shapes = vec![Shape::Scalar];
        
//         // Generate vectors
//         shapes.extend((2..=self.max_dim).map(Shape::Vector));
        
//         // Generate matrices
//         for rows in 2..=self.max_dim {
//             for cols in 2..=self.max_dim {
//                 shapes.push(Shape::Matrix(rows, cols));
//             }
//         }
        
//         shapes
//     }
    
//     pub fn compatible_shapes(&self, shape: &Shape) -> Vec<Shape> {
//         // Define which shapes can be combined with each other
//         match shape {
//             Shape::Scalar => self.generate_all_shapes(),
//             Shape::Vector(n) => vec![Shape::Scalar, Shape::Vector(*n)],
//             Shape::Matrix(r, c) => vec![Shape::Scalar, Shape::Vector(*c), Shape::Matrix(*r, *c)],
//         }
//     }
// }