use super::SharedString;

#[derive(Debug)]
#[repr(C)]
pub struct ProtectedString {
    pub shared_source: SharedString,
    pub shared_bytecode: SharedString,
}
