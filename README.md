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




## todo 

future: use macros to populate the NonTerminalGrammar to make the user API experience better 

https://rust-unofficial.github.io/patterns/additional_resources/design-principles.html

Investigate https://rust-unofficial.github.io/patterns/functional/optics.html to possibly redefine a better type system

What's left (roughly estimated):

Fitness & Evolution (~6-8 hours)
  - Basic fitness functions (MSE, etc.) - done
  - Simple mutation (subtree replacement)
  - Basic crossover (subtree swapping)
  - Population management

Polish & Testing (~4-6 hours)
  - Integration testing
  - Bug fixes
  - Maybe add a few unary operations (sin/cos)

Documentation/Examples (~2-4 hours)
  - Clean up the API
  - Add some example problems