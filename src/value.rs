use std::ffi::{CStr, CString};

use cyberscript_sys::{clAsBool, clAsFloat, clAsInteger, clToTempString, CLValue};

use crate::Vm;

#[derive(Debug, Default, Copy, Clone)]
pub struct Value {
    pub inner: CLValue,
}

impl Value {
    pub fn to_temp_string(&self, vm: &mut Vm) -> CString {
        unsafe {
            let cl = clToTempString(vm.inner, self.inner);
            let st = CStr::from_ptr(cl.ptr);

            st.to_owned()
        }
    }
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
