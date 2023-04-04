#[derive(Debug)]
#[repr(C)]
pub struct Atomic<T> {
    pub v: T,
}
