#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Pair<T1, T2> {
    first: T1,
    second: T2,
}
