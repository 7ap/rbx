use std::ffi::c_char;
use std::mem;
use std::ptr::NonNull;

use anyhow::{anyhow, Result};
use pelite::pattern;
use pelite::pe64::{Pe, PeView};
use windows::core::*;
use windows::Win32::System::LibraryLoader::*;

use crate::app::util::CopyOnWritePtr;
use crate::stl::memory::SharedPtr;
use crate::stl::vector::Vector;

use super::TaskSchedulerJob;

#[derive(Debug)]
#[repr(C)]
pub struct TaskScheduler {
    _pad0: [c_char; 0x0178],
    pub all_jobs: CopyOnWritePtr<Vector<SharedPtr<TaskSchedulerJob>>>,
    pub currently_running_jobs: CopyOnWritePtr<Vector<SharedPtr<TaskSchedulerJob>>>,
}

impl TaskScheduler {
    pub fn get() -> Result<NonNull<TaskScheduler>> {
        const SIGNATURE: &str = "48 83 EC 28 8B 0D ? ? ? ? 65 48 8B 04 25 58 00 00 00 BA 64 01 00 00 48 8B 0C C8 8B 04 0A 39 05 ? ? ? ? 7F 2B 48 8B 05 ? ? ? ? 48 83 C4 28 C3 48 89 05 ? ? ? ? 48 8D 0D ? ? ? ? E8 ? ? ? ? 48 8B 05 ? ? ? ? 48 83 C4 28 C3 48 8D 0D ? ? ? ? E8 ? ? ? ? 83 3D ? ? ? ? ? 75 C0 B9 68 03";

        let base = unsafe { GetModuleHandleA(PCSTR(mem::zeroed()))? };
        let view = unsafe { PeView::module(base.0 as _) };

        let scanner = view.scanner();
        let pattern = pattern::parse(SIGNATURE)?;

        let mut save = [0];
        if !scanner.finds_code(&pattern, &mut save) {
            return Err(anyhow!("failed to find task scheduler"));
        }

        let task_scheduler = unsafe {
            mem::transmute::<usize, unsafe extern "cdecl" fn() -> *mut TaskScheduler>(
                base.0 as usize + save[0] as usize,
            )()
        };

        Ok(NonNull::new(task_scheduler).unwrap())
    }

    pub fn get_jobs_info(&self) -> Vec<NonNull<TaskSchedulerJob>> {
        let mut jobs = Vec::new();

        let all_jobs = &self.all_jobs.object;
        let (mut first, last) = (all_jobs.first, all_jobs.last);

        while first != last {
            unsafe {
                if let Some(job) = NonNull::new((*first).ptr) {
                    jobs.push(job);
                }

                first = first.add(1);
            };
        }

        jobs
    }

    pub fn get_jobs_by_name(&self, name: &str) -> Result<NonNull<TaskSchedulerJob>> {
        let jobs = self.get_jobs_info();

        for job in jobs {
            if unsafe { job.as_ref().name.r_str()? } == name {
                return Ok(job);
            }
        }

        Err(anyhow!("could not find job"))
    }
}
