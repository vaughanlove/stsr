// use crate::types::{DataType::{self, *}, Shape::{self, *}};
// use crate::ops::Operation;
// use rand::Rng;
// use std::any::Any;

// pub struct TypeRegistry;

// impl TypeRegistry {
//     pub fn get_compatible_shape_and_type(data_type: DataType, shape: Shape) -> Vec<(DataType, Shape)> {
//         match (data_type, shape) {
//             (Integer, Scalar) => vec![(Integer, Scalar), (Float, Scalar), (Integer, Vector(2)), (Integer, Vector(3)), (Float, Matrix(2, 2)), (Integer, Vector(2))],
//             _ => vec![]
//         }
//     }

//     pub fn create_random_terminal(data_type: DataType, shape: Shape) -> Box<dyn Any> {
//         let mut rng = rand::rng();

//         match (data_type, shape) {
//             (DataType::Integer, Shape::Scalar) => {
//                 let value: i64 = rng.random_range(-100..=100);
//                 Box::new(value)
//             }
//             (DataType::Float, Shape::Scalar) => {
//                 let value: f64 = rng.random_range(-100.0..=100.0);
//                 Box::new(value)
//             }
//             (DataType::Integer, Shape::Vector(size)) => {
//                 let value: Vec<i64> = (0..size).map(|_| rng.random_range(-100..=100)).collect();
//                 Box::new(value)
//             }
//             (DataType::Float, Shape::Vector(size)) => {
//                 let value: Vec<f64> = (0..size).map(|_| rng.random_range(-100.0..=100.0)).collect();
//                 Box::new(value)
//             }
//             (DataType::Integer, Shape::Matrix(rows, cols)) => {
//                 let value: Vec<Vec<i64>> = (0..rows)
//                     .map(|_| (0..cols).map(|_| rng.random_range(-100..=100)).collect())
//                     .collect();
//                 Box::new(value)
//             }
//             (DataType::Float, Shape::Matrix(rows, cols)) => {
//                 let value: Vec<Vec<f64>> = (0..rows)
//                     .map(|_| (0..cols).map(|_| rng.random_range(-100.0..=100.0)).collect())
//                     .collect();
//                 Box::new(value)
//             }
//         }
//     }

//     pub fn create_variable_terminal(data_type: DataType, shape: Shape) -> Box<dyn Any> {
//         // For variable terminals, we store a placeholder value
//         // The actual value will come from the VariableContext during evaluation
//         match (data_type, shape) {
//             (DataType::Integer, Shape::Scalar) => Box::new(0i64),
//             (DataType::Float, Shape::Scalar) => Box::new(0.0f64),
//             (DataType::Integer, Shape::Vector(size)) => Box::new(vec![0i64; size]),
//             (DataType::Float, Shape::Vector(size)) => Box::new(vec![0.0f64; size]),
//             (DataType::Integer, Shape::Matrix(rows, cols)) => Box::new(vec![vec![0i64; cols]; rows]),
//             (DataType::Float, Shape::Matrix(rows, cols)) => Box::new(vec![vec![0.0f64; cols]; rows]),
//         }
//     }

//     pub fn extract_terminal(
//         value: &Box<dyn Any>,
//         data_type: DataType,
//         shape: Shape,
//     ) -> Result<Box<dyn Any>, String> {
//         match (data_type, shape) {
//             (DataType::Integer, Shape::Scalar) => {
//                 let val = value
//                     .downcast_ref::<i64>()
//                     .ok_or("Failed to downcast terminal i64")?;
//                 Ok(Box::new(*val))
//             }
//             (DataType::Float, Shape::Scalar) => {
//                 let val = value
//                     .downcast_ref::<f64>()
//                     .ok_or("Failed to downcast terminal f64")?;
//                 Ok(Box::new(*val))
//             }
//             (DataType::Integer, Shape::Vector(_)) => {
//                 let val = value
//                     .downcast_ref::<Vec<i64>>()
//                     .ok_or("Failed to downcast terminal Vec<i64>")?;
//                 Ok(Box::new(val.clone()))
//             }
//             (DataType::Float, Shape::Vector(_)) => {
//                 let val = value
//                     .downcast_ref::<Vec<f64>>()
//                     .ok_or("Failed to downcast terminal Vec<f64>")?;
//                 Ok(Box::new(val.clone()))
//             }
//             (DataType::Integer, Shape::Matrix(_, _)) => {
//                 let val = value
//                     .downcast_ref::<Vec<Vec<i64>>>()
//                     .ok_or("Failed to downcast terminal Vec<Vec<i64>>")?;
//                 Ok(Box::new(val.clone()))
//             }
//             (DataType::Float, Shape::Matrix(_, _)) => {
//                 let val = value
//                     .downcast_ref::<Vec<Vec<f64>>>()
//                     .ok_or("Failed to downcast terminal Vec<Vec<f64>>")?;
//                 Ok(Box::new(val.clone()))
//             }
//         }
//     }

//     pub fn execute_operation(
//         operation: Operation,
//         left_type: (DataType, Shape),
//         right_type: (DataType, Shape),
//         left_val: Box<dyn Any>,
//         right_val: Box<dyn Any>,
//     ) -> Result<Box<dyn Any>, String> {
//         match (operation, left_type, right_type) {
//             // Integer + Integer -> Integer
//             (
//                 Operation::Add,
//                 (DataType::Integer, Shape::Scalar),
//                 (DataType::Integer, Shape::Scalar),
//             ) => {
//                 let left = left_val
//                     .downcast::<i64>()
//                     .map_err(|_| "Failed to downcast left i64")?;
//                 let right = right_val
//                     .downcast::<i64>()
//                     .map_err(|_| "Failed to downcast right i64")?;
//                 Ok(Box::new(*left + *right))
//             }

//             // Float + Float -> Float
//             (
//                 Operation::Add,
//                 (DataType::Float, Shape::Scalar),
//                 (DataType::Float, Shape::Scalar),
//             ) => {
//                 let left = left_val
//                     .downcast::<f64>()
//                     .map_err(|_| "Failed to downcast left f64")?;
//                 let right = right_val
//                     .downcast::<f64>()
//                     .map_err(|_| "Failed to downcast right f64")?;
//                 Ok(Box::new(*left + *right))
//             }

//             // Integer - Integer -> Integer
//             (
//                 Operation::Subtract,
//                 (DataType::Integer, Shape::Scalar),
//                 (DataType::Integer, Shape::Scalar),
//             ) => {
//                 let left = left_val
//                     .downcast::<i64>()
//                     .map_err(|_| "Failed to downcast left i64")?;
//                 let right = right_val
//                     .downcast::<i64>()
//                     .map_err(|_| "Failed to downcast right i64")?;
//                 Ok(Box::new(*left - *right))
//             }

//             // Float - Float -> Float
//             (
//                 Operation::Subtract,
//                 (DataType::Float, Shape::Scalar),
//                 (DataType::Float, Shape::Scalar),
//             ) => {
//                 let left = left_val
//                     .downcast::<f64>()
//                     .map_err(|_| "Failed to downcast left f64")?;
//                 let right = right_val
//                     .downcast::<f64>()
//                     .map_err(|_| "Failed to downcast right f64")?;
//                 Ok(Box::new(*left - *right))
//             }

//             // Integer * Integer -> Integer
//             (
//                 Operation::Multiply,
//                 (DataType::Integer, Shape::Scalar),
//                 (DataType::Integer, Shape::Scalar),
//             ) => {
//                 let left = left_val
//                     .downcast::<i64>()
//                     .map_err(|_| "Failed to downcast left i64")?;
//                 let right = right_val
//                     .downcast::<i64>()
//                     .map_err(|_| "Failed to downcast right i64")?;
//                 Ok(Box::new(*left * *right))
//             }

//             // Float * Float -> Float
//             (
//                 Operation::Multiply,
//                 (DataType::Float, Shape::Scalar),
//                 (DataType::Float, Shape::Scalar),
//             ) => {
//                 let left = left_val
//                     .downcast::<f64>()
//                     .map_err(|_| "Failed to downcast left f64")?;
//                 let right = right_val
//                     .downcast::<f64>()
//                     .map_err(|_| "Failed to downcast right f64")?;
//                 Ok(Box::new(*left * *right))
//             }

//             // Integer / Integer -> Integer
//             (
//                 Operation::Divide,
//                 (DataType::Integer, Shape::Scalar),
//                 (DataType::Integer, Shape::Scalar),
//             ) => {
//                 let left = left_val
//                     .downcast::<i64>()
//                     .map_err(|_| "Failed to downcast left i64")?;
//                 let right = right_val
//                     .downcast::<i64>()
//                     .map_err(|_| "Failed to downcast right i64")?;
//                 if *right == 0 {
//                     return Err("Division by zero".to_string());
//                 }
//                 Ok(Box::new(*left / *right))
//             }

//             // Float / Float -> Float
//             (
//                 Operation::Divide,
//                 (DataType::Float, Shape::Scalar),
//                 (DataType::Float, Shape::Scalar),
//             ) => {
//                 let left = left_val
//                     .downcast::<f64>()
//                     .map_err(|_| "Failed to downcast left f64")?;
//                 let right = right_val
//                     .downcast::<f64>()
//                     .map_err(|_| "Failed to downcast right f64")?;
//                 if *right == 0.0 {
//                     return Err("Division by zero".to_string());
//                 }
//                 Ok(Box::new(*left / *right))
//             }

//             _ => Err(format!(
//                 "Unsupported operation: {:?} between {:?} and {:?}",
//                 operation, left_type, right_type
//             )),
//         }
//     }
// }