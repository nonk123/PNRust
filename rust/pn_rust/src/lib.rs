//! The PNRust library.
//!
//! This package provides types and functions shared across all PNRust modules.

mod buffer;
mod value;

pub use crate::buffer::Buffer;
pub use crate::value::Value;

use std::sync::mpsc;
use std::sync::Mutex;

use lazy_static::lazy_static;

lazy_static! {
    /// This variable is set inside `pn_rust_dll`. DO NOT TOUCH.
    pub static ref GML_RESULT: Mutex<Option<Value>> = Mutex::new(None);
}

/// A global function that can be exported to GML.
///
/// The function's arguments are stored in a buffer. It may read as many as it wants or can.
///
/// [Context::call_gml] allows to seamlessly call GML functions from Rust.
pub type ExportedFunction = fn(&mut Context, &mut Buffer) -> Value;

/// A module that can be attached to a PNRust-powered game.
///
/// See the project's README for more info.
pub trait Module {
    fn new() -> Self;

    fn init(&mut self, _context: &mut Context) {}

    /// Generate this function using the [exports!] macro.
    fn exports(
        &mut self,
        context: &mut Context,
        function_name: &str,
        arguments: &mut Buffer,
    ) -> Option<Value>;
}

#[macro_export]
macro_rules! exports {
    ($($name:ident),+) => {
        fn exports(
            &mut self,
            context: &mut Context,
            function_name: &str,
            arguments: &mut Buffer,
        ) -> Option<Value> {
            match function_name {
                $(stringify!($name) => Some(self.$name(context, arguments))),*,
                _ => None,
            }
        }
    };
}

pub type GMLCall = (String, Vec<Value>);
pub type RustCall = (String, Buffer);

pub struct Context {
    pub gml_channel: Option<Mutex<mpsc::Sender<GMLCall>>>,
    pub rust_channel: Option<Mutex<mpsc::Sender<RustCall>>>,
}

impl Context {
    /// Call a GML function with specified name and arguments.
    pub fn call_gml(&mut self, function_name: &str, arguments: Vec<Value>) -> Value {
        *GML_RESULT.lock().unwrap() = None;

        self.gml_channel
            .as_ref()
            .unwrap()
            .lock()
            .unwrap()
            .send((function_name.to_string(), arguments))
            .unwrap();

        loop {
            let result = GML_RESULT.lock().unwrap();

            if result.is_some() {
                return result.clone().unwrap();
            }
        }
    }
}
