use crate::stl::string::String as CxxString;

#[repr(C)]
pub struct Name {
    pub str: CxxString,
}
