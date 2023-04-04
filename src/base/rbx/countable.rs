pub mod diagnostics {
    use std::ffi::c_char;
    use std::marker::PhantomData;

    #[derive(Debug)]
    #[repr(C)]
    pub struct Countable<T> {
        t: PhantomData<T>,
        _todo: c_char,
    }
}
