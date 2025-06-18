use crate::{ops::Operation, types::TypeInfo};

/// implementation of non-terminals

// I think the idea here is that some sort of type registry will determine if the inputs can ever correspond to the output (based on the op).
struct NonTerminal {
    inputs: Vec<TypeInfo>,
    output: TypeInfo,
    operation: Operation 
}
#[derive(Debug)]

pub struct NonTerminalRule {
    pub input_one_type: TypeInfo,
    pub input_two_type: TypeInfo,
    pub operation: Operation,
    pub output: TypeInfo,
    pub func: fn(&dyn std::any::Any, &dyn std::any::Any) -> Box<dyn std::any::Any>,
}

impl NonTerminalRule {
    pub fn new(
        input_one_type: TypeInfo, 
        input_two_type: TypeInfo, 
        operation: Operation, 
        output: TypeInfo,
        func: fn(&dyn std::any::Any, &dyn std::any::Any) -> Box<dyn std::any::Any>
    ) -> Self {
        NonTerminalRule {
            input_one_type,
            input_two_type,
            operation,
            output,
            func
        }
    }

    /// Helper to create scalar arithmetic rules
    pub fn scalar_arithmetic(
        data_type: crate::types::DataType,
        operation: Operation,
        func: fn(&dyn std::any::Any, &dyn std::any::Any) -> Box<dyn std::any::Any>
    ) -> Self {
        let scalar_type = TypeInfo { 
            shape: crate::types::Shape::Scalar, 
            data_type: data_type 
        };
        Self::new(scalar_type, scalar_type, operation, scalar_type, func)
    }

    /// Execute the operation with the given inputs
    pub fn execute(&self, input1: &dyn std::any::Any, input2: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        (self.func)(input1, input2)
    }
}

#[derive(Debug)]
/// meant to be user-defined
pub struct NonTerminalGrammar {
    pub rules: Vec<NonTerminalRule>
}

impl NonTerminalGrammar {
    pub fn new() -> Self {
        NonTerminalGrammar {
            rules: Vec::new()
        }
    }

    pub fn add_rule(&mut self, rule: NonTerminalRule) {
        let swapped = NonTerminalRule {
            input_one_type: rule.input_two_type,
            input_two_type: rule.input_one_type,
            output: rule.output,
            operation: rule.operation,
            func: rule.func
        };

        // a rule should have both left/right combos.
        if (rule.input_one_type != rule.input_two_type) {
            self.rules.push(swapped);
        }
        self.rules.push(rule);
    }

    pub fn get_all_possible_input_types_with_operations(&self, output_type: TypeInfo) -> Vec<(TypeInfo, TypeInfo, Operation)> {
        let mut temp: Vec<(TypeInfo, TypeInfo, Operation)> = Vec::new();

        for rule in &self.rules {
            if rule.output == output_type {
                temp.push((rule.input_one_type, rule.input_two_type, rule.operation))
            }
        }

        temp
    }
}


// todo: macro for instantiating the non terminal rules
// #[macro_export]
// macro_rules! make_nonterminal_ruleset {
//     ($($e:expr), *) => {
//         let temp_vec = Vec::new();
//         ($(
//             $element:expr

//         ),*)
//     };
// }
