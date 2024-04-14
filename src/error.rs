use cyberscript_sys::{CL_ERROR_COMPILE, CL_ERROR_PANIC, CL_ERROR_UNKNOWN};

#[derive(Debug)]
#[repr(i32)]
pub enum Error {
    Compile = CL_ERROR_COMPILE,
    Panic = CL_ERROR_PANIC,
    Unknown = CL_ERROR_UNKNOWN,
}
