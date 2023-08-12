#[derive(Debug)]
#[repr(C)]
pub struct RunningAverage<T1, T2> {
    pub lerp: f64,
    pub last_sample_value: T1,
    pub average_value: T2,
    pub average_variance: T2,
    pub first_time: bool,
}

#[derive(Debug)]
#[repr(C)]
pub struct RunningAverageTimeInterval {
    pub timer: Time,
    pub first_time: bool,
    pub average: RunningAverage<f64, f64>,
    pub ignore_time_since_last_sample: *mut bool,
}

#[derive(Debug)]
#[repr(C)]
pub struct RunningAverageDutyCycle {
    pub time: RunningAverage<f64, f64>,
    pub interval: RunningAverageTimeInterval,
}

#[derive(Debug)]
#[repr(C)]
pub struct Time {
    pub sec: f64,
}

#[derive(Debug)]
#[repr(C)]
pub struct TimeInterval {
    pub sec: f64,
}

#[derive(Debug)]
#[repr(C)]
pub struct Timer {
    pub start: Time,
}
