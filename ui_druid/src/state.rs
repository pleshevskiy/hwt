use druid::{Data, Lens};
use std::rc::Rc;

#[derive(Clone, Data, Lens)]
pub struct App {
    pub micro_break: BreakTimer,
    pub rest_break: BreakTimer,
    pub notifier: Timer,
}

#[derive(Clone, Data, Lens)]
pub struct BreakTimer {
    pub timer: Timer,
}

#[derive(Clone, Data, Lens)]
pub struct Timer {
    pub durations: Rc<Vec<u32>>,
    pub progress: f64,
    pub time: String,
}

impl Timer {
    pub fn single(duration: u32) -> Self {
        Self::multiple(vec![duration])
    }

    pub fn multiple(durations: Vec<u32>) -> Self {
        Self {
            durations: Rc::new(durations),
            progress: Default::default(),
            time: Default::default(),
        }
    }
}
