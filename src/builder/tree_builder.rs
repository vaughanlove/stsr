
pub struct TreeBuilder {
    // TERMINAL and FUNCTION sets. These have embedded lookup tables for generating safe syntax trees.
    terminal_set: TerminalSet,
    function_set: FunctionSet, 

    // One modification vs. original GP is that we need to specify the output type.
    output_type: TerminalShape,


    // How deep can the tree initialization be?
    max_initial_tree_depth: usize

    
}

