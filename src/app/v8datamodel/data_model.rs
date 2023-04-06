use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;

use anyhow::Result;

use crate::app::v8tree::Instance;
use crate::base::rbx::TaskScheduler;

#[derive(Debug)]
#[repr(C)]
pub struct DataModel {
    _super0: Instance,
}

impl Deref for DataModel {
    type Target = Instance;

    fn deref(&self) -> &Self::Target {
        &self._super0
    }
}

impl DerefMut for DataModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self._super0
    }
}

impl DataModel {
    pub fn get() -> Result<NonNull<DataModel>> {
        let data_model = unsafe {
            let task_scheduler = TaskScheduler::get()?.as_mut();
            let render = task_scheduler.get_jobs_by_name("Render")?.as_mut();

            NonNull::new(render.arbiter.ptr.byte_offset(0x10) as *mut DataModel)
        };

        Ok(data_model.unwrap())
    }
}
