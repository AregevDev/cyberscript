use cyberscript_sys::{clAsBool, clAsFloat, clAsInteger, CLValue};

#[derive(Debug, Default, Copy, Clone)]
pub struct Value {
    pub inner: CLValue,
}

impl From<Value> for bool {
    fn from(value: Value) -> Self {
        unsafe { clAsBool(value.inner) }
    }
}

impl From<Value> for i64 {
    fn from(value: Value) -> Self {
        unsafe { clAsInteger(value.inner) }
    }
}

impl From<Value> for f64 {
    fn from(value: Value) -> Self {
        unsafe { clAsFloat(value.inner) }
    }
}
