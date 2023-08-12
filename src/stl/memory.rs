use std::ops::Deref;

use super::atomic::AtomicCounter;

#[derive(Debug)]
#[repr(C)]
pub struct RefCountBase {
    pub uses: AtomicCounter,
    pub weaks: AtomicCounter,
}

#[derive(Debug)]
#[repr(C)]
pub struct PtrBase<T> {
    pub ptr: *mut T,
    pub rep: *mut RefCountBase,
}

#[derive(Debug)]
#[repr(C)]
pub struct SharedPtr<T> {
    _super0: PtrBase<T>,
}

impl<T> Deref for SharedPtr<T> {
    type Target = PtrBase<T>;

    fn deref(&self) -> &Self::Target {
        &self._super0
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct UniquePtr<T> {
    ptr: *mut T,
}
