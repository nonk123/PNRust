//! # Introduction
//!
//! "Abandon all your hope ye who enter here."
//!
//! This is the Rust filling for PNRust. See the project's README for info on how to get started.
//!
//! # Implementation
//!
//! Under the hood, PNRust employs two different threads: the GML thread and the Rust thread.
//!
//! The algorithm for calling Rust functions from GML is as follows:
//!
//! ## The Main Thread (GML→Rust)
//!
//! 1. The GML code calls `rs_call`, which calls Rust's `call_function`. This sends a message to the Rust thread.
//! 2. The Rust thread receives the message and, in parallel, executes the steps described in the next section.
//! 3. GML waits for a response from either the Rust thread or the GML thread in a loop.
//! 4. If the response came from the GML thread, a GML function is called with specified arguments. The result is then passed back to the GML thread. The loop continues.
//! 5. If the response came from the Rust thread, it contains the result of the GML→Rust call. It is returned from `rs_call`.
//!
//! ## The Rust Thread (Rust→Rust)
//!
//! 1. The Rust thread receives a message from `call_function`. It contains the name of the Rust function and a buffer with its arguments.
//! 2. The Rust thread looks for the function in the list of exported functions and panics if no function is found.
//! 3. The exported Rust function is called with specified arguments.
//! 4. At any time of its execution, it may call `call_gml`. It sends a message to the GML thread and blocks until the main thread responds. See the next section for more info.
//!
//! ## The GML Thread (Rust→GML)
//!
//! There is only one step to this thread's workings: it sends a message back to the main thread, which contains the GML function's name and arguments.

use lazy_static::lazy_static;

mod buffer;
mod exports;
mod value;

use crate::buffer::Buffer;
use crate::value::Value;
use libc::c_char;
use libc::c_double;
use std::collections::HashMap;
use std::ffi::CStr;
use std::sync::atomic::AtomicPtr;
use std::sync::mpsc;
use std::sync::Mutex;

/// A global function that can be exported to GML.
///
/// The function's arguments are stored in a buffer. It may read as many as it wants or can.
///
/// [Context::call_gml] allows to seamlessly call GML functions from Rust.
pub type ExportedFunction = fn(&mut Context, &mut Buffer) -> Value;

type GMLCall = (String, Vec<Value>);
type RustCall = (String, Buffer);

pub struct Context {
    exports: HashMap<String, Box<ExportedFunction>>,
    gml_channel: Option<Mutex<mpsc::Sender<GMLCall>>>,
    rust_channel: Option<Mutex<mpsc::Sender<RustCall>>>,
}

impl Context {
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

    pub fn export(&mut self, name: &str, function: ExportedFunction) {
        self.exports.insert(name.to_string(), Box::new(function));
    }
}

lazy_static! {
    static ref GML_RESULT: Mutex<Option<Value>> = Mutex::new(None);
    static ref CONTEXT: Mutex<Context> = Mutex::new(Context {
        exports: HashMap::new(),
        gml_channel: None,
        rust_channel: None,
    });
}

#[no_mangle]
pub unsafe extern "cdecl" fn init_gml_executor(ptr: *mut c_char, signal: *mut c_char) -> c_double {
    let mut ptr = AtomicPtr::new(ptr);
    let mut signal = AtomicPtr::new(signal);

    let (sender, receiver) = mpsc::channel();

    CONTEXT.lock().unwrap().gml_channel = Some(Mutex::new(sender));

    std::thread::spawn(move || loop {
        let (function_name, arguments) = receiver.recv().unwrap();

        let mut buffer = Buffer::new();
        buffer.write(&Value::String(function_name));
        buffer.write(&Value::Array(arguments));
        buffer.copy_into(&ptr.get_mut());

        let mut buffer = Buffer::new();
        buffer.write_byte(&1);
        buffer.copy_into(&signal.get_mut());
    });

    1.0
}

#[no_mangle]
pub unsafe extern "cdecl" fn init_rust_executor(ptr: *mut c_char, signal: *mut c_char) -> c_double {
    let mut ptr = AtomicPtr::new(ptr);
    let mut signal = AtomicPtr::new(signal);

    let (sender, receiver) = mpsc::channel();

    CONTEXT.lock().unwrap().rust_channel = Some(Mutex::new(sender));

    std::thread::spawn(move || loop {
        let (function_name, mut arguments) = receiver.recv().unwrap();

        let result = {
            let mut context = CONTEXT.lock().unwrap();
            let wrapper = &context.exports[&function_name].clone();
            wrapper(&mut context, &mut arguments)
        };

        let mut buffer = Buffer::new();
        buffer.write(&result);
        buffer.copy_into(&ptr.get_mut());

        let mut buffer = Buffer::new();
        buffer.write_byte(&2);
        buffer.copy_into(&signal.get_mut());
    });

    1.0
}

#[no_mangle]
pub unsafe extern "cdecl" fn call_function(
    name: *const c_char,
    arguments: *const c_char,
) -> c_double {
    let function_name = CStr::from_ptr(name).to_str().unwrap().to_string();
    let arguments = Buffer::from_ptr(arguments);

    // Send a RustCall over context's Rust channel.
    CONTEXT
        .lock()
        .unwrap()
        .rust_channel
        .as_ref()
        .unwrap()
        .lock()
        .unwrap()
        .send((function_name, arguments))
        .unwrap();

    1.0
}

#[no_mangle]
pub unsafe extern "cdecl" fn receive_result(ptr: *const c_char) -> c_double {
    let mut buffer = Buffer::from_ptr(ptr);
    *GML_RESULT.lock().unwrap() = Some(buffer.read());
    1.0
}

#[no_mangle]
pub extern "cdecl" fn init_exports() -> c_double {
    exports::init_exports(&mut CONTEXT.lock().unwrap());

    1.0
}
