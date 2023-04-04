#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Vector<T> {
    pub first: *mut T,
    pub last: *mut T,
    pub end: *mut T,
}
