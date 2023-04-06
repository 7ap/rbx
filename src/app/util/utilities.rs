use std::ffi::c_char;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct CopyOnWritePtr<Class> {
    pub object: Class,
    _pad0: c_char,
}
