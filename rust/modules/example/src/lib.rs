// A module has the same name as its root directory.

use pn_rust::{exports, Buffer, Context, Module, Value};

// The entry point of any module is a struct named after it.
pub struct Example;

impl Module for Example {
    exports![solve_quadratic];

    fn new() -> Self {
        Self
    }
}

impl Example {
    fn solve_quadratic(&mut self, context: &mut Context, args: &mut Buffer) -> Value {
        let a = args.read().to_real();
        let b = args.read().to_real();
        let c = args.read().to_real();

        let d = b * b - 4.0 * a * c;

        let roots = if d < 0.0 {
            vec![]
        } else if d.abs() <= 1e-5 {
            vec![Value::Real(-b / (2.0 * a))]
        } else {
            let sqrt = d.sqrt();

            vec![
                Value::Real((-b + sqrt) / (2.0 * a)),
                Value::Real((-b - sqrt) / (2.0 * a)),
            ]
        };

        context.call_gml("print_roots", roots);

        Value::Undefined
    }
}
