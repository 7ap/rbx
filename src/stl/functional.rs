use std::ffi::c_char;
use std::marker::PhantomData;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct EqualTo<Key> {
    key: PhantomData<Key>,
    _todo: c_char,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Function<Fn> {
    function: PhantomData<Fn>,
    _todo: [c_char; 32],
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Hash<Key> {
    key: PhantomData<Key>,
    _todo: c_char,
}
