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

    pub fn to_string(self) -> Option<String> {
        match self {
            Self::String(string) => Some(string),
            _ => None,
        }
    }

    pub fn to_real(self) -> Option<f64> {
        match self {
            Self::Real(real) => Some(real),
            _ => None,
        }
    }

    pub fn to_array(self) -> Option<Vec<Value>> {
        match self {
            Self::Array(array) => Some(array),
            _ => None,
        }
    }
}
