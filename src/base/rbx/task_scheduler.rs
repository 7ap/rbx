use std::ffi::c_char;

use crate::app::util::CopyOnWritePtr;
use crate::stl::memory::SharedPtr;
use crate::stl::vector::Vector;

use super::TaskSchedulerJob;

#[derive(Debug)]
#[repr(C)]
pub struct TaskScheduler {
    _pad0: [c_char; 0x0178],
    all_jobs: CopyOnWritePtr<Vector<SharedPtr<TaskSchedulerJob>>>,
    currently_running_jobs: CopyOnWritePtr<Vector<SharedPtr<TaskSchedulerJob>>>,
}
