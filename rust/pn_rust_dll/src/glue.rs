//! Example glue file for the future module system.
//!
//! The glue file is generated automatically alongside a Cargo.toml for `pn_rust_dll`.

use pn_rust::{Buffer, Context, Module, Value};

use lazy_static::lazy_static;

use example::Example;

use std::sync::Mutex;

lazy_static! {
    static ref EXAMPLE: Mutex<Example> = Mutex::new(Example::new());
}

pub fn init(context: &mut Context) {
    EXAMPLE.lock().unwrap().init(context);
}

pub fn call_function(
    context: &mut Context,
    function_name: String,
    arguments: &mut Buffer,
) -> Value {
    if let Some(result) = EXAMPLE
        .lock()
        .unwrap()
        .exports(context, &function_name, arguments)
    {
        return result;
    }

    Value::Undefined
}
