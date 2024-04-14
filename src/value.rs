use cyberscript_sys::{clAsInteger, CLValue};

#[derive(Debug, Default, Copy, Clone)]
pub struct Value {
    pub inner: CLValue,
}

impl From<Value> for i64 {
    fn from(value: Value) -> Self {
        unsafe { clAsInteger(value.inner) }
    }
}
