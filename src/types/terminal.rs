use std::fmt;

/// Primitive type definition
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PrimitiveType {
    Integer,
    Float,
    Boolean,
    Void, 
}
/// Generic param representation
/// ie in the paper, when a terminal is defined as VECTOR-GENNUM1
/// This part initially confused me, but it is so that during tree gen these vectors can be any size, while still guarenteeing that 
/// the tree has valid options to generate from.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GenericParam {
    Name(String),
}


/// The shape that the terminal 
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TerminalShape {
    Scalar(PrimitiveType),
    Vector(PrimitiveType, GenericParam),  
    Matrix(PrimitiveType, GenericParam, GenericParam),  
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Terminal {
    Variable(TerminalShape),
    Constant(TerminalShape)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BasicFunction {
    Add,
    Subtract,
    Divide,
    Multiply,
    Log,
    Dot,
    Exp,
    Root,
}

// For generic functions
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GenericFunction {
    pub name: String,
    pub function: BasicFunction,
    pub inputs: Vec<TerminalShape>,  // Vec allows for any number of terminal inputs
    pub output: TerminalShape,
    pub generic_params: Vec<GenericParam>,  // Track which generic params this function uses
}

// impl GenericFunction {
//     fn can_be_instantiated_with_args(&self)
// }

impl fmt::Display for PrimitiveType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PrimitiveType::Integer => write!(f, "INT"),
            PrimitiveType::Float => write!(f, "FLOAT"),
            PrimitiveType::Boolean => write!(f, "BOOL"),
            PrimitiveType::Void => write!(f, "VOID"),
        }
    }
}

impl fmt::Display for GenericParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GenericParam::Name(name) => write!(f, "{}", name),
        }
    }
}

impl fmt::Display for TerminalShape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TerminalShape::Scalar(prim_type) => write!(f, "{}", prim_type),
            TerminalShape::Vector(prim_type, dim) => write!(f, "{}-VECTOR-{}", prim_type, dim),
            TerminalShape::Matrix(prim_type, rows, cols) => 
                write!(f, "{}-MATRIX-{}-{}", prim_type, rows, cols),
        }
    }
}

impl fmt::Display for Terminal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Terminal::Variable(shape) => write!(f, "Variable({})", shape),
            Terminal::Constant(shape) => write!(f, "Constant({})", shape),
        }
    }
}

impl fmt::Display for BasicFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}


impl fmt::Display for GenericFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {} with and outputs {}", self.name, self.function, self.output)
    }
}