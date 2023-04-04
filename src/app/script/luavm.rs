use std::mem;

use crate::stl::string::String as CxxString;

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub enum CompileTarget {
    Client = 0,
    CoreScript = 1,
    Studio = 2,
    CoreScriptRaw = 3,
}

pub fn compile(
    result: *mut CxxString,
    _source: *const CxxString,
    _compile_target: CompileTarget,
    _env: i32,
) -> *mut CxxString {
    unsafe {
        (&mut *result).data.buf = mem::zeroed();
        (&mut *result).size = 0;
        (&mut *result).res = 0xf;
    }

    result
}
