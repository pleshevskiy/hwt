use crate::config;
use druid::{Color, Env, Key};

pub const TIMER_DURATION: Key<f64> = Key::new("hwt.env.comp.timer.duration");

pub const MICRO_BREAK_TIMER_DURATION: Key<f64> = Key::new("hwt.env.widget.micro_break.duration");
pub const MICRO_BREAK_TIMER_POSTPONE_DURATION: Key<f64> =
    Key::new("hwt.env.widget.micro_break.postpone_duration");
pub const MICRO_BREAK_TIMER_REST_DURATION: Key<f64> =
    Key::new("hwt.env.widget.micro_break.rest_duration");

pub const REST_BREAK_TIMER_DURATION: Key<f64> = Key::new("hwt.env.widget.rest_break.duration");
pub const REST_BREAK_TIMER_POSTPONE_DURATION: Key<f64> =
    Key::new("hwt.env.widget.rest_break.postpone_duration");
pub const REST_BREAK_TIMER_REST_DURATION: Key<f64> =
    Key::new("hwt.env.widget.rest_break.rest_duration");

pub const BREAK_NOTIFIER_TIMER_DURATION: Key<f64> = Key::new("hwt.env.widget.notifier.duration");

pub const WIN_REST_AUTO_RESTART_BREAK_TIMERS: Key<bool> = Key::new("hwt.env.rest.auto_restart");

pub fn configure(env: &mut Env, config: &config::Env) {
    let col_def_white = hsl(0.0, 0.0, 1.0);
    // let col_def_black = hsl(0.0, 0.0, 0.0);
    let col_graphite = hsl(0.0, 0.0, 0.13);
    let col_pale = hsl(0.0, 0.0, 0.86);
    let col_faded = hsl(0.0, 0.0, 0.96);

    // druid theme
    env.set(druid::theme::WINDOW_BACKGROUND_COLOR, col_def_white);
    env.set(druid::theme::BACKGROUND_DARK, col_pale.clone());
    env.set(druid::theme::BACKGROUND_LIGHT, col_faded.clone());
    env.set(druid::theme::BORDER_DARK, col_pale.clone());
    env.set(druid::theme::BORDER_LIGHT, col_faded.clone());
    env.set(druid::theme::LABEL_COLOR, col_graphite);
    env.set(druid::theme::BUTTON_DARK, col_pale);
    env.set(druid::theme::BUTTON_LIGHT, col_faded);

    // timers
    env.set(MICRO_BREAK_TIMER_DURATION, config.micro_timer.duration);
    env.set(
        MICRO_BREAK_TIMER_POSTPONE_DURATION,
        config.micro_timer.postpone_duration,
    );
    env.set(
        MICRO_BREAK_TIMER_REST_DURATION,
        config.micro_timer.rest_duration,
    );

    env.set(REST_BREAK_TIMER_DURATION, config.rest_timer.duration);
    env.set(
        REST_BREAK_TIMER_POSTPONE_DURATION,
        config.rest_timer.postpone_duration,
    );
    env.set(
        REST_BREAK_TIMER_REST_DURATION,
        config.rest_timer.rest_duration,
    );

    env.set(BREAK_NOTIFIER_TIMER_DURATION, config.notifier.duration);

    env.set(
        WIN_REST_AUTO_RESTART_BREAK_TIMERS,
        config.auto_restart.unwrap_or_default(),
    );
}

#[allow(clippy::many_single_char_names)]
fn hsl(h: f64, s: f64, l: f64) -> Color {
    hsla(h, s, l, 1.0)
}

#[allow(clippy::many_single_char_names)]
fn hsla(h: f64, s: f64, l: f64, a: f64) -> Color {
    let (r, g, b) = hsl_to_rgb(h, s, l);
    Color::rgba(r, g, b, a)
}

#[allow(clippy::many_single_char_names)]
fn hsl_to_rgb(h: f64, s: f64, l: f64) -> (f64, f64, f64) {
    if s == 0.0 {
        (l, l, l)
    } else {
        let q = if l < 0.5 {
            l * (1.0 + s)
        } else {
            l + s - l * s
        };

        let p = 2.0 * l - q;
        (
            hue_to_rgb(p, q, h + 1.0 / 3.0),
            hue_to_rgb(p, q, h),
            hue_to_rgb(p, q, h - 1.0 / 3.0),
        )
    }
}

fn hue_to_rgb(p: f64, q: f64, t: f64) -> f64 {
    let mut t = t;
    if t < 0.0 {
        t += 1.0;
    }
    if t > 1.0 {
        t -= 1.0;
    };
    if t < 1.0 / 6.0 {
        return p + (q - p) * 6.0 * t;
    }
    if t < 1.0 / 2.0 {
        return q;
    }
    if t < 2.0 / 3.0 {
        return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
    }

    p
}
