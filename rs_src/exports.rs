// The #[allow(...)] flags disable certain lints made by the compiler.
//
// Make sure to remove them once you start using the variables and types.

#[allow(unused_imports)]
use crate::buffer::Buffer;

#[allow(unused_imports)]
use crate::value::Value;

use crate::Context;

pub fn init_exports(#[allow(unused_variables)] context: &mut Context) {
    // context.export("my_function", my_function);
}

// Write your functions here in the form:
//
// ```
// pub fn my_function(context: &mut Context, args: &mut Buffer) -> Value {
//     ...
// }
// ```
