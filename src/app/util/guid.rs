use std::ffi::c_char;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Guid {
    _todo: [c_char; 8],
}

#[derive(Debug)]
pub struct GuidItem<T> {
    t: PhantomData<T>,
    _todo: [c_char; 10],
}
