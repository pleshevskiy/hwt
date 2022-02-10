use crate::state;
use druid::{Env, Key};

pub const TIMER_DURATION: Key<f64> = Key::new("hwt.env.components.timer.duration");
pub const TIMER_POSTPONE_DURATION: Key<f64> =
    Key::new("hwt.env.components.timer.postpone_duration");

pub const MICRO_BREAK_TIMER_DURATION: Key<f64> = Key::new("hwt.env.widget.micro_break.duration");
pub const MICRO_BREAK_TIMER_POSTPONE_DURATION: Key<f64> =
    Key::new("hwt.env.widget.micro_break.postpone_duration");
pub const REST_BREAK_TIMER_DURATION: Key<f64> = Key::new("hwt.env.widget.rest_break.duration");
pub const REST_BREAK_TIMER_POSTPONE_DURATION: Key<f64> =
    Key::new("hwt.env.widget.rest_break.postpone_duration");

pub const BREAK_NOTIFIER_TIMER_DURATION: Key<f64> = Key::new("hwt.env.widget.notifier.duration");

pub fn configure(env: &mut Env, _data: &state::App) {
    // log::info!("Env: {:?}", env.get_all().collect::<Vec<_>>());

    env.set(MICRO_BREAK_TIMER_DURATION, 5.0);
    env.set(MICRO_BREAK_TIMER_POSTPONE_DURATION, 2.5);

    log::info!("{}", env.get(MICRO_BREAK_TIMER_DURATION));

    env.set(REST_BREAK_TIMER_DURATION, 60.0 * 45.0);
    env.set(REST_BREAK_TIMER_POSTPONE_DURATION, 60.0 * 2.5);

    env.set(BREAK_NOTIFIER_TIMER_DURATION, 5.0);
}
