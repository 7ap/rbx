pub mod diagnostics {
    use std::marker::PhantomData;

    #[derive(Debug)]
    #[repr(C)]
    pub struct Countable<T> {
        t: PhantomData<T>,
    }
}
