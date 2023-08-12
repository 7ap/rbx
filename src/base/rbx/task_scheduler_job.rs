use crate::stl::condition_variable::ConditionVariable;
use crate::stl::memory::SharedPtr;
use crate::stl::string::String as StlString;

use super::{RunningAverageDutyCycle, TaskSchedulerArbiter, Time, TimeInterval};

#[derive(Debug)]
#[repr(C)]
pub enum JobPriority {
    EarlyRendering,
    NetworkReceiveIncoming,
    NetworkPostReplicatorCreation,
    NetworkProcessIncoming,
    NetworkProcessPhysicsReceive,
    NetworkProcessJoin,
    Default,
    Simulation,
    Heartbeat,
    NetworkStreamingSolver,
    NetworkMegaOutgoing,
    NetworkProcessOutgoing,
    NetworkSendOutgoing,
    Render,
    NetworkDisconnectCleanUp,
    NotOrderDependent,
}

#[derive(Debug)]
#[repr(C)]
pub enum TaskSchedulerJobState {
    Unknown,
    Running,
}

#[derive(Debug)]
#[repr(C)]
pub struct TaskSchedulerJob {
    pub vmt: usize,
    pub this: SharedPtr<Self>,
    pub name: StlString,
    pub arbiter: SharedPtr<TaskSchedulerArbiter>,
    pub priority: JobPriority,
    pub step_start_time: Time,
    pub step_stop_time: Time,
    pub state: TaskSchedulerJobState,
    pub time_of_last_step: Time,
    pub timespan_of_last_step: TimeInterval,
    pub duty_cycle: RunningAverageDutyCycle,
    pub job_workers: i32,
    pub should_notify_completion: bool,
    _pad0: [u8; 0x003],
    pub completion_cond: ConditionVariable,
}

#[derive(Debug)]
#[repr(C)]
pub struct SharedJobPointerOrder;
