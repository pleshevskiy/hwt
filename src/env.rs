use crate::state;
use druid::{Env, Key};

pub const TIMER_INIT_DURATION: Key<f64> = Key::new("hwt.env.comp.timer.init_duration");
pub const TIMER_DURATION: Key<f64> = Key::new("hwt.env.comp.timer.duration");
pub const TIMER_POSTPONE_DURATION: Key<f64> = Key::new("hwt.env.comp.timer.postpone_duration");
pub const TIMER_WAIT_DURATION: Key<f64> = Key::new("hwt.env.comp.timer.wait_duration");

pub const MICRO_BREAK_TIMER_DURATION: Key<f64> = Key::new("hwt.env.widget.micro_break.duration");
pub const MICRO_BREAK_TIMER_POSTPONE_DURATION: Key<f64> =
    Key::new("hwt.env.widget.micro_break.postpone_duration");
pub const MICRO_BREAK_TIMER_WAIT_DURATION: Key<f64> =
    Key::new("hwt.env.widget.micro_break.wait_duration");

pub const REST_BREAK_TIMER_DURATION: Key<f64> = Key::new("hwt.env.widget.rest_break.duration");
pub const REST_BREAK_TIMER_POSTPONE_DURATION: Key<f64> =
    Key::new("hwt.env.widget.rest_break.postpone_duration");
pub const REST_BREAK_TIMER_WAIT_DURATION: Key<f64> =
    Key::new("hwt.env.widget.rest_break.wait_duration");

pub const BREAK_NOTIFIER_TIMER_DURATION: Key<f64> = Key::new("hwt.env.widget.notifier.duration");

pub fn configure(env: &mut Env, _data: &state::App) {
    env.set(MICRO_BREAK_TIMER_DURATION, 10.0);
    env.set(MICRO_BREAK_TIMER_POSTPONE_DURATION, 10.0);
    env.set(MICRO_BREAK_TIMER_WAIT_DURATION, 30.0);

    env.set(REST_BREAK_TIMER_DURATION, mins(45.0));
    env.set(REST_BREAK_TIMER_POSTPONE_DURATION, mins(5.0));
    env.set(REST_BREAK_TIMER_WAIT_DURATION, mins(10.0));

    env.set(BREAK_NOTIFIER_TIMER_DURATION, 10.0);
}

fn mins(m: f64) -> f64 {
    return m * 60.0;
}
