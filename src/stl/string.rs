use std::fmt::Debug;

#[derive(Clone, Copy)]
#[repr(C)]
pub union StringVal {
    pub buf: [u8; 16],
    pub ptr: *mut u8,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct String {
    pub data: StringVal,
    pub size: usize,
    pub res: usize,
}

impl Debug for String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("TODO")
    }
}
