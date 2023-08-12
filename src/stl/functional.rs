use std::marker::PhantomData;

#[derive(Debug)]
#[repr(C)]
pub struct Function<Fn> {
    function: PhantomData<Fn>,
    _todo: [u8; 0x018],
}
