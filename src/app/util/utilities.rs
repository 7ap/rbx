use crate::stl::memory::SharedPtr;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct CopyOnWritePtr<Class> {
    pub object: SharedPtr<Class>,
}
