use druid::{Data, Lens, WidgetId};

pub const NOTIFIER_TIMER: WidgetId = WidgetId::reserved(2);

#[derive(Clone, Data, Lens)]
pub struct App {
    pub micro_break: BreakTimer,
    pub rest_break: BreakTimer,
    pub notifier: BreakTimer,
}

#[derive(Clone, Data, Lens)]
pub struct BreakTimer {
    pub duration: u32,
    pub progress: f64,
    pub time: String,
}

impl BreakTimer {
    pub fn calculate_progress(&self, elapsed: std::time::Duration) -> f64 {
        (elapsed.as_secs_f64() / self.duration as f64).min(1.0)
    }

    pub fn create_time_string(&self, elapsed: std::time::Duration) -> String {
        let all_secs = (self.duration as i64) - (elapsed.as_secs() as i64);
        let sign = if all_secs < 0 { "-" } else { "" };
        let all_secs = all_secs.abs();

        let mins = all_secs / 60;
        let secs = all_secs % 60;
        format!(
            "{}{}:{}{}",
            sign,
            mins,
            if secs < 10 { "0" } else { "" },
            secs
        )
    }
}
