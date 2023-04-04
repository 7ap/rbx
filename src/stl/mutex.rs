use std::ffi::c_char;

#[derive(Debug)]
#[repr(C)]
pub struct Mutex {
    _todo: [c_char; 80],
}
