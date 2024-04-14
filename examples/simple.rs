use std::ffi::CStr;

use cyberscript::{get_full_version, Vm};
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
    println!("--- {} ---", get_full_version());

    let mut vm = Vm::new().unwrap();
    vm.set_printer(cy_print);

    let src = c"var a = 'hello'\nreturn a";

    match vm.eval(src) {
        Ok(v) => {
            let va = v.to_temp_string(&mut vm);
            println!("{}", va.to_str().unwrap());
        }
        Err(_e) => {
            let msg = vm.last_error_summary().unwrap();
            println!("{}", msg.to_str().unwrap());
        }
    }
}
