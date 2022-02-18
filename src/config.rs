use serde::Deserialize;

#[derive(Deserialize)]
pub struct Env {
    pub micro_timer: BreakTimer,
    pub rest_timer: BreakTimer,
    pub notifier: Notifier,
}

#[derive(Deserialize)]
pub struct BreakTimer {
    pub duration: f64,
    pub postpone_duration: f64,
    pub rest_duration: f64,
}

#[derive(Deserialize)]
pub struct Notifier {
    pub duration: f64,
}
