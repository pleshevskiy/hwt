use druid::{Data, Lens};
use std::ops::Div;
use std::time::Duration;

#[derive(Clone, Data, Lens)]
pub struct App {
    pub paused: bool,
    pub micro_break: BreakTimer,
    pub rest_break: BreakTimer,
    pub notifier: Timer,
}

#[derive(Clone, Data, Lens)]
pub struct BreakTimer {
    pub work_timer: Timer,
}

impl BreakTimer {
    pub fn new() -> Self {
        Self {
            work_timer: Timer::new(),
        }
    }
}

#[derive(Clone, Data, Lens)]
pub struct Timer {
    pub progress: f64,
    pub time: String,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            progress: Default::default(),
            time: Default::default(),
        }
    }

    pub fn reset(&mut self, duration: Duration) {
        self.update_progress_and_time(Duration::ZERO, duration, duration)
    }

    pub fn update_progress_and_time(
        &mut self,
        elapsed: Duration,
        duration: Duration,
        full_duration: Duration,
    ) {
        self.update_progress(elapsed, full_duration);
        self.update_time(elapsed, duration);
    }

    fn update_progress(&mut self, elapsed: Duration, duration: Duration) {
        self.progress = elapsed.as_secs_f64().div(duration.as_secs_f64()).min(1.0);
    }

    fn update_time(&mut self, elapsed: Duration, duration: Duration) {
        let all_secs = duration.as_secs_f64() - elapsed.as_secs_f64();
        let sign = if all_secs < 0.0 { "-" } else { "" };
        let all_secs = all_secs.abs();

        let mins = all_secs / 60.0;
        let secs = all_secs % 60.0;
        self.time = format!(
            "{}{}:{}{}",
            sign,
            mins as i32,
            if secs < 10.0 { "0" } else { "" },
            secs as u32
        );
    }
}
