use crate::app::util::Name;

#[derive(Debug)]
#[repr(C)]
pub struct Descriptor {
    _vtable: *mut usize,
    pub name: *const Name,
    pub attributes: Attributes,
}

#[derive(Debug)]
#[repr(C)]
pub struct Attributes {
    pub is_deprecated: bool,
    pub preferred: *const Descriptor,
    pub thread_safety: ThreadSafety,
}

#[derive(Debug)]
#[repr(C)]
pub enum ThreadSafety {
    Unset = 0,
    Unsafe = 1,
    ReadSafe = 3,
    LocalSafe = 7,
    Safe = 15,
}
