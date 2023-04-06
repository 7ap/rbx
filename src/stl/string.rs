use std::ffi::{c_char, CStr};
use std::fmt::Debug;
use std::slice;
use std::string::String as RustString;

use anyhow::Result;

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
        f.write_str(self.r_str().unwrap().as_str())
    }
}

impl String {
    pub fn new(string: &'static str) -> Self {
        if string.len() >= 16 {
            panic!("this deosnt work");
        }

        let buf = string.as_ptr() as *mut [u8; 16];

        let data = StringVal {
            buf: unsafe { (*buf).clone() },
        };

        Self {
            data,
            size: string.len(),
            res: 0xf + string.len(),
        }
    }

    pub fn c_str(&self) -> Result<*const c_char> {
        let string = if self.size < 16 {
            let bytes = unsafe { self.data.buf.as_slice() };
            CStr::from_bytes_until_nul(bytes)?
        } else {
            let bytes = unsafe { slice::from_raw_parts(self.data.ptr, self.size + 1) };
            CStr::from_bytes_until_nul(bytes).unwrap()
        };

        Ok(string.as_ptr())
    }

    pub fn r_str(&self) -> Result<RustString> {
        let string = unsafe { CStr::from_ptr(self.c_str()?) };

        Ok(RustString::from_utf8_lossy(string.to_bytes()).to_string())
    }
}
