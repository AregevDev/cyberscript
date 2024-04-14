mod error;
mod value;
mod vm;

pub use crate::error::*;
pub use crate::value::*;
pub use crate::vm::*;
pub use cyberscript_sys as sys;

use std::ffi::CStr;

// Top level functions
pub fn get_full_version() -> &'static str {
    unsafe {
        CStr::from_ptr(sys::clGetFullVersion().ptr)
            .to_str()
            .unwrap()
    }
}

pub fn get_version() -> &'static str {
    unsafe { CStr::from_ptr(sys::clGetVersion().ptr).to_str().unwrap() }
}

pub fn get_build() -> &'static str {
    unsafe { CStr::from_ptr(sys::clGetBuild().ptr).to_str().unwrap() }
}

pub fn get_commit() -> &'static str {
    unsafe { CStr::from_ptr(sys::clGetCommit().ptr).to_str().unwrap() }
}
