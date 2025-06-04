/// Some of the Terminal instances are variables. 
/// This file defines a lookup table for those variable names and their values.
/// Type information is held in the Terminal itself.

use std::collections::HashMap;
use std::any::Any;

struct Variable {
    name: String,
    value: Box<dyn Any>
}

type Variables = HashMap<String, Variable>;


