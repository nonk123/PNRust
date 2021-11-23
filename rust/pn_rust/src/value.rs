#[derive(Clone)]
pub enum Value {
    Undefined,
    String(String),
    Real(f64),
    Array(Vec<Value>),
}

impl Value {
    pub fn is_undefined(&self) -> bool {
        match self {
            Self::Undefined => true,
            _ => false,
        }
    }

    pub fn as_string(self) -> Option<String> {
        match self {
            Self::String(string) => Some(string),
            _ => None,
        }
    }

    pub fn to_string(self) -> String {
        self.as_string().expect("expected a string")
    }

    pub fn as_real(self) -> Option<f64> {
        match self {
            Self::Real(real) => Some(real),
            _ => None,
        }
    }

    pub fn to_real(self) -> f64 {
        self.as_real().expect("expected a real value")
    }

    pub fn as_array(self) -> Option<Vec<Value>> {
        match self {
            Self::Array(array) => Some(array),
            _ => None,
        }
    }

    pub fn to_array(self) -> Vec<Value> {
        self.as_array().expect("expected an array")
    }
}
