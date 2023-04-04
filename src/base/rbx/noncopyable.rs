use std::ffi::c_char;

#[derive(Debug)]
#[repr(C)]
pub struct Noncopyable {
    _todo: c_char,
}
