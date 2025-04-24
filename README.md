Implementation of https://dl.acm.org/doi/pdf/10.1162/evco.1995.3.2.199 in rust as a general-purpose STGP framework. 

### notes for implementation

the definition of what consitutes a legal parse tree has a few additional criteria beyond standard genetic programming
1. the root node of the tree returns a value of the type required by the problem
2. each nonroot node returns a value of the type required by the parent node as an argument.