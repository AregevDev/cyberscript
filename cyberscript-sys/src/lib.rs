#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

use std::ffi::CStr;
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

impl From<&CStr> for CLStr {
    fn from(value: &CStr) -> Self {
        let ptr = value.as_ptr();

        CLStr {
            ptr,
            len: value.to_bytes().len(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::{CStr, CString};

    #[test]
    fn it_works() {
        unsafe {
            let vm = clCreate();
            let cstr = CString::new(include_str!("../scripts/test.cy")).unwrap();

            let mut value: CLValue = 0;
            let res = clEval(
                vm,
                CLStr {
                    ptr: cstr.as_ptr(),
                    len: cstr.to_bytes().len(),
                },
                &mut value as *mut CLValue,
            );

            if res == CL_SUCCESS {
                assert_eq!(clAsInteger(value), 60);
                clRelease(vm, value);
            } else {
                let err = clNewLastErrorSummary(vm);
                println!("{}", CStr::from_ptr(err.ptr).to_str().unwrap());
                clFreeStrZ(vm, err.ptr);
            }

            clDestroy(vm);
        }
    }
}
