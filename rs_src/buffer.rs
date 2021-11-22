use crate::value::Value;

use libc::c_char;

const BUFFER_SIZE: usize = 65535;

#[derive(Clone)]
pub struct Buffer {
    contents: [u8; BUFFER_SIZE],
    position: usize,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            contents: [0; BUFFER_SIZE],
            position: 0,
        }
    }

    pub fn rewind(&mut self) {
        self.position = 0;
    }

    pub fn read(&mut self) -> Value {
        let value_type = self.read_byte();

        match value_type {
            0 => Value::Undefined,
            1 => Value::String(self.read_string()),
            2 => Value::Real(self.read_real()),
            3 => Value::Array(self.read_array()),
            _ => unreachable!(),
        }
    }

    pub fn write(&mut self, value: &Value) {
        match value {
            Value::Undefined => self.write_undefined(),
            Value::String(string) => self.write_string(string),
            Value::Real(real) => self.write_real(real),
            Value::Array(array) => self.write_array(array),
        }
    }

    fn read_len(&mut self) -> i32 {
        let length_bytes = [
            self.read_byte(),
            self.read_byte(),
            self.read_byte(),
            self.read_byte(),
        ];

        i32::from_le_bytes(length_bytes)
    }

    fn read_byte(&mut self) -> u8 {
        let byte = self.contents[self.position];
        self.position += 1;
        byte
    }

    fn read_string(&mut self) -> String {
        let length = self.read_len();
        let mut result = String::new();

        for _ in 0..length {
            result.push(self.read_byte() as char);
        }

        result
    }

    fn read_real(&mut self) -> f64 {
        let bytes = [
            self.read_byte(),
            self.read_byte(),
            self.read_byte(),
            self.read_byte(),
            self.read_byte(),
            self.read_byte(),
            self.read_byte(),
            self.read_byte(),
        ];

        f64::from_le_bytes(bytes)
    }

    fn read_array(&mut self) -> Vec<Value> {
        let len = self.read_len();

        let mut array = Vec::new();
        array.resize(len as usize, Value::Undefined);

        for i in 0..len {
            array[i as usize] = self.read();
        }

        array
    }

    fn write_len(&mut self, len: i32) {
        for byte in len.to_le_bytes() {
            self.write_byte(&byte);
        }
    }

    fn write_undefined(&mut self) {
        self.write_byte(&0);
    }

    pub fn write_byte(&mut self, value: &u8) {
        if self.position == BUFFER_SIZE {
            panic!("Buffer has run out of space");
        }

        self.contents[self.position] = *value;
        self.position += 1;
    }

    fn write_string(&mut self, string: &String) {
        let bytes = string.as_bytes();

        self.write_byte(&1);
        self.write_len(bytes.len() as i32);

        for byte in bytes {
            self.write_byte(byte);
        }
    }

    fn write_real(&mut self, real: &f64) {
        self.write_byte(&2);

        for byte in real.to_le_bytes() {
            self.write_byte(&byte);
        }
    }

    fn write_array(&mut self, array: &Vec<Value>) {
        self.write_byte(&3);
        self.write_len(array.len() as i32);

        for element in array {
            self.write(element);
        }
    }

    pub unsafe fn copy_into(&self, destination: &*mut c_char) {
        for i in 0..BUFFER_SIZE {
            *destination.add(i) = self.contents[i] as c_char;
        }
    }

    pub unsafe fn from_ptr(contents: *const c_char) -> Self {
        let mut buffer = Self::new();

        for i in 0..BUFFER_SIZE {
            buffer.write_byte(&(*contents.add(i) as u8));
        }

        buffer.rewind();

        buffer
    }
}
