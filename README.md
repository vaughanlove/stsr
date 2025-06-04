Implementation of https://dl.acm.org/doi/pdf/10.1162/evco.1995.3.2.199 in rust as a general-purpose STGP framework. 

### notes for implementation

the definition of what consitutes a legal parse tree has a few additional criteria beyond standard genetic programming
1. the root node of the tree returns a value of the type required by the problem
2. each nonroot node returns a value of the type required by the parent node as an argument.



- Each variable and constant has an assigned type. ie, variable with name 'V1' can be of type 'VECTOR-3' (3 dimensional vector.)
- Each function has a specified type for each argument and for the value it returns.
    - INPUT TYPES / OUTPUT TYPES



note to self:
because of how rust works, the syntax tree that I create will need to be fully instantiated to the absolute maximum size at the start, to ensure all types can fit in each node.


if we define a trait "Node", then a vector can dynamically size using:

Vec<Box<dyn Node>> - which can be interpreted as Vector of Boxes of any type that implement the trait Node



note - 