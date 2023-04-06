#[derive(Debug)]
#[repr(C)]
pub struct UniqueSessionId {
    pub bits_hi: u32,
    pub bits_lo: u32,
    pub timestamp: u32,
}

#[derive(Debug)]
#[repr(C)]
pub struct UniqueId {
    pub session_id: UniqueSessionId,
    pub index: u32,
}
