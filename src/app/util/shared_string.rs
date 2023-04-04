//! TODO: Incomplete.

#[derive(Debug)]
#[repr(C)]
pub struct SharedString {
    flyweight: *mut *mut usize,
    read_count: i32,
}
