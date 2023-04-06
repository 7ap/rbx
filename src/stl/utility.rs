#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Pair<T1, T2> {
    pub first: T1,
    pub second: T2,
}
