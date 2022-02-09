use druid::{Data, Lens, WidgetId};

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
}
