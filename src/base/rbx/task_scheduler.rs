use crate::app::util::OrderedVectorSet;
use crate::stl::atomic::Atomic;
use crate::stl::condition_variable::ConditionVariable;
use crate::stl::functional::Function;
use crate::stl::memory::{SharedPtr, UniquePtr};
use crate::stl::mutex::Mutex;
use crate::stl::vector::Vector;

use super::{
    AsyncTaskQueue, ParallelJobQueue, RunningAverage, RunningAverageDutyCycle,
    SharedJobPointerOrder, TaskQueue, TaskSchedulerJob, Time, TimeInterval,
};

#[derive(Debug)]
#[repr(C)]
pub struct TaskScheduler {
    pub cycle_counter: Atomic<u32>,
    pub cycle_throttling: Atomic<bool>,
    _pad0: [u8; 0x003],
    pub scheduler_duty_cycle: RunningAverageDutyCycle,
    pub task_frame_time_average: RunningAverage<f64, f64>,
    pub previous_frame_time: f64,
    pub step_timer: Time,
    pub max_time_frame: f64,
    pub global_mutex: Mutex,
    pub last_cycle_timestamp: Time,
    pub workers_wakeup_condition: ConditionVariable,
    pub next_cycle_timepoint: [u8; 0x008], // i cba man
    pub cycle_interval: TimeInterval,
    pub background_mode: bool,
    pub cycle_interval_to_restore: TimeInterval,
    pub parallel_job_queue: UniquePtr<ParallelJobQueue>,
    pub all_jobs: OrderedVectorSet<TaskSchedulerJob, SharedJobPointerOrder>,
    pub currently_running_jobs: OrderedVectorSet<TaskSchedulerJob, SharedJobPointerOrder>,
    pub workers_desired_count: i32,
    pub workers_active_count: i32,
    pub workers_stopped_cond: ConditionVariable,
    pub default_task_queue: SharedPtr<TaskQueue>,
    pub globally_exclusive_tasks: Vector<Function<fn()>>,
    pub async_task_queue: AsyncTaskQueue,
    pub is_shutting_down: Atomic<bool>,
    pub should_flip_profiler_frame: Atomic<bool>,
}
