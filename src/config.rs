use serde::Deserialize;

pub fn default() -> Env {
    Env {
        micro_timer: BreakTimer {
            duration: mins(5.0),
            postpone_duration: mins(2.5),
            rest_duration: mins(0.5),
        },
        rest_timer: BreakTimer {
            duration: mins(45.0),
            postpone_duration: mins(5.0),
            rest_duration: mins(10.0),
        },
        notifier: Notifier { duration: 10.0 },
        auto_restart: Some(false),
    }
}

fn mins(m: f64) -> f64 {
    m * 60.0
}

#[derive(Deserialize)]
pub struct Env {
    pub micro_timer: BreakTimer,
    pub rest_timer: BreakTimer,
    pub notifier: Notifier,
    pub auto_restart: Option<bool>,
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
