use druid::{Data, Lens, WidgetId};
use std::time::Duration;

pub const MICRO_WORK_TIMER_WIDGET_ID: WidgetId = WidgetId::reserved(1);
pub const REST_WORK_TIMER_WIDGET_ID: WidgetId = WidgetId::reserved(2);

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
    pub wait_timer: Timer,
}

impl BreakTimer {
    pub fn new(work_duration: u32, wait_duration: u32) -> Self {
        Self {
            work_timer: Timer::new(work_duration),
            wait_timer: Timer::new(wait_duration),
        }
    }
}

#[derive(Clone, Data, Lens)]
pub struct Timer {
    pub duration: u32,
    pub progress: f64,
    pub time: String,
}

impl Timer {
    pub fn new(duration: u32) -> Self {
        Self {
            duration,
            progress: Default::default(),
            time: Default::default(),
        }
    }

    pub fn reset(&mut self) {
        self.update_progress_and_time(Duration::new(0, 0))
    }

    pub fn update_progress_and_time(&mut self, elapsed: Duration) {
        self.update_progress(elapsed);
        self.update_time(elapsed);
    }

    fn update_progress(&mut self, elapsed: Duration) {
        self.progress = (elapsed.as_secs_f64() / self.duration as f64).min(1.0);
    }

    fn update_time(&mut self, elapsed: Duration) {
        let all_secs = (self.duration as i64) - (elapsed.as_secs() as i64);
        let sign = if all_secs < 0 { "-" } else { "" };
        let all_secs = all_secs.abs();

        let mins = all_secs / 60;
        let secs = all_secs % 60;
        self.time = format!(
            "{}{}:{}{}",
            sign,
            mins,
            if secs < 10 { "0" } else { "" },
            secs
        );
    }
}
