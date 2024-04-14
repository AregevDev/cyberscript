use std::ffi::{CStr, CString};

use cyberscript_sys::{
    clCreate, clDestroy, clEval, clFreeStr, clNewLastErrorSummary, clSetPrinter, CLStr, CLVM,
};

use crate::error::Error;
use crate::value::Value;

#[derive(Debug)]
pub struct Vm<'a> {
    pub inner: &'a mut CLVM,
}

impl<'a> Vm<'a> {
    pub fn new() -> Result<Vm<'a>, Error> {
        let raw = unsafe { clCreate() };
        let p = unsafe { raw.as_mut() }.ok_or(Error::Unknown)?;

        Ok(Vm { inner: p })
    }

    pub fn set_printer(&mut self, f: extern "C" fn(*mut CLVM, CLStr)) {
        unsafe { clSetPrinter(self.inner, Some(f)) };
    }

    pub fn eval(&mut self, src: &CStr) -> Result<Value, Error> {
        let mut out_val = Value::default();
        let code = unsafe { clEval(self.inner, CLStr::from(src), &mut out_val.inner) };

        if code != 0 {
            return Err(Error::Unknown);
        }

        Ok(out_val)
    }

    pub fn last_error_summary(&mut self) -> Option<CString> {
        unsafe {
            let err = clNewLastErrorSummary(self.inner);
            if err.ptr.is_null() {
                return None;
            }

            let st = CStr::from_ptr(err.ptr).to_owned();
            let clone = st.clone();
            clFreeStr(self.inner, err);

            Some(clone)
        }
    }
}

impl<'a> Drop for Vm<'a> {
    fn drop(&mut self) {
        unsafe { clDestroy(self.inner) }
    }
}
