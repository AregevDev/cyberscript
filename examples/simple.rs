use std::ffi::CStr;

use cyberscript::Vm;
use cyberscript_sys::{CLStr, CLVM};

extern "C" fn cy_print(_vm: *mut CLVM, st: CLStr) {
    unsafe {
        if *st.ptr != '\n' as i8 {
            let rs_str = CStr::from_ptr(st.ptr).to_str().unwrap();
            println!("[cyberscript] {}", rs_str);
        }
    }
}

fn main() {
    let mut vm = Vm::new().unwrap();
    vm.set_printer(cy_print);

    let src = c"var a = 13\nprint a\nreturn a * a";

    match vm.eval(src) {
        Ok(v) => {
            let v: i64 = v.into();
            println!("{}", v);
        }
        Err(_e) => {}
    }
}
