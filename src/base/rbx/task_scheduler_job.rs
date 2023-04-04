use std::ops::Deref;

use crate::stl::memory::SharedPtr;
use crate::stl::string::String as CxxString;

use super::TaskSchedulerArbiter;

#[derive(Debug)]
#[repr(C)]
pub struct TaskSchedulerJob {
    _vtable: *mut usize,
    _super0: SharedPtr<Self>,
    pub name: CxxString,
    pub arbriter: SharedPtr<TaskSchedulerArbiter>,
}

impl Deref for TaskSchedulerJob {
    type Target = SharedPtr<Self>;

    fn deref(&self) -> &Self::Target {
        &self._super0
    }
}
