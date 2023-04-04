use std::ffi::c_char;
use std::marker::PhantomData;
use std::ops::Deref;

use super::atomic::*;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct RefCountBase {
    pub uses: AtomicCounter,
    pub weaks: AtomicCounter,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct PtrBase<T> {
    pub ptr: *mut T,
    pub rep: *mut RefCountBase,
}

#[derive(Clone, Copy, Debug)]
pub struct SharedPtr<T> {
    _super0: PtrBase<T>,
}

impl<T> Deref for SharedPtr<T> {
    type Target = PtrBase<T>;

    fn deref(&self) -> &Self::Target {
        &self._super0
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct DefaultDelete<T> {
    t: PhantomData<T>,
    _todo: c_char,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct UniquePtr<T> {
    pair: (DefaultDelete<T>, *mut T),
}
