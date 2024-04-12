#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::{CStr, CString};

    #[test]
    fn it_works() {
        unsafe {
            let vm = csCreate();
            let cstr = CString::new(include_str!("../scripts/test.cy")).unwrap();

            let mut value: CsValue = 0;
            let res = csEval(
                vm,
                CsStr {
                    buf: cstr.as_ptr(),
                    len: cstr.to_bytes().len(),
                },
                &mut value as *mut CsValue,
            );

            if res == CS_SUCCESS {
                assert_eq!(csAsInteger(value), 60);
                csRelease(vm, value);
            } else {
                let err = csNewLastErrorSummary(vm);
                println!("{}", CStr::from_ptr(err.buf).to_str().unwrap());
                csFreeStrZ(vm, err.buf);
            }

            csDestroy(vm);
        }
    }
}
