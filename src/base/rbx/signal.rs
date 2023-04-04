//! TODO

use std::marker::PhantomData;

pub mod signals {
    #[derive(Debug)]
    #[repr(C)]
    pub struct ScopedConnection {
        _todo: usize,
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct Signal<Signature> {
    signature: PhantomData<Signature>,
    _todo: usize,
}
