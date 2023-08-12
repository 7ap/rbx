pub type AtomicCounter = u64;

#[derive(Debug)]
#[repr(C)]
pub struct Atomic<T> {
    pub value: T,
}
