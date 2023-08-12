#[derive(Debug)]
#[repr(C)]
pub struct ParallelJobQueue {}

#[derive(Debug)]
#[repr(C)]
pub struct TaskQueue {}

#[derive(Debug)]
#[repr(C)]
pub struct AsyncTaskQueue {
    _todo: [u8; 0x130],
}
