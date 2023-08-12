use std::ffi::CStr;
use std::ptr;
use std::slice;
use std::string::String as RustString;

#[repr(C)]
union StringValue {
    small: [u8; 0x10],
    large: *mut u8,
}

impl std::fmt::Debug for StringValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TODO")
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct String {
    val: StringValue,
    len: usize,
    cap: usize,
}

impl String {
    pub fn new() -> Self {
        Self {
            val: StringValue { small: [0; 0x10] },
            len: 0,
            cap: 0xf,
        }
    }

    pub fn assign(&mut self, s: &str) {
        if s.len() < 0x10 {
            self.cap = 0xf;

            unsafe {
                ptr::copy_nonoverlapping(s.as_ptr(), self.val.small.as_mut_ptr(), s.len());

                if self.len >= 0x10 {
                    libc::free(self.val.large as *mut _);
                }
            }

            self.len = s.len();
        } else {
            self.cap = s.len() + 1;

            unsafe {
                self.val.large = libc::malloc(self.cap) as _;
                ptr::copy_nonoverlapping(s.as_ptr(), self.val.large, s.len());
                *self.val.large.add(s.len()) = 0;
            }

            self.len = s.len();
        }
    }

    pub fn data(&self) -> RustString {
        let string = if self.len < 0x10 {
            let bytes = unsafe { self.val.small.as_slice() };
            CStr::from_bytes_until_nul(bytes).unwrap()
        } else {
            let bytes = unsafe { slice::from_raw_parts(self.val.large, self.len + 1) };
            CStr::from_bytes_until_nul(bytes).unwrap()
        };

        RustString::from(string.to_str().unwrap())
    }
}

impl Drop for String {
    fn drop(&mut self) {
        if self.len >= 0x10 {
            unsafe {
                libc::free(self.val.large as _);
            }
        }
    }
}

impl From<&str> for String {
    fn from(s: &str) -> Self {
        let mut string = Self::new();
        string.assign(s);
        string
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string() {
        let _ = env_logger::builder().is_test(true).try_init();

        let mut string = String::new();
        info!("empty: {:?} ({:?})", string.data(), string);
        assert_eq!("", string.data());

        string.assign("Hello, world!");
        info!("small: {:?} ({:?})", string.data(), string);
        assert_eq!("Hello, world!", string.data());

        string.assign("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        info!("large: {:?} ({:?})", string.data(), string);
        assert_eq!("ABCDEFGHIJKLMNOPQRSTUVWXYZ", string.data());
    }
}
