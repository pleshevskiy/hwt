use crate::state;
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

pub fn configure(env: &mut Env, _data: &state::App) {
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
    env.set(MICRO_BREAK_TIMER_DURATION, 10.0);
    env.set(MICRO_BREAK_TIMER_POSTPONE_DURATION, 10.0);
    env.set(MICRO_BREAK_TIMER_REST_DURATION, 30.0);

    env.set(REST_BREAK_TIMER_DURATION, mins(45.0));
    env.set(REST_BREAK_TIMER_POSTPONE_DURATION, mins(5.0));
    env.set(REST_BREAK_TIMER_REST_DURATION, mins(10.0));

    env.set(BREAK_NOTIFIER_TIMER_DURATION, 10.0);
}

fn mins(m: f64) -> f64 {
    return m * 60.0;
}

fn hsl(h: f64, s: f64, l: f64) -> Color {
    hsla(h, s, l, 1.0)
}

fn hsla(h: f64, s: f64, l: f64, a: f64) -> Color {
    let (r, g, b) = hsl_to_rgb(h, s, l);
    Color::rgba(r, g, b, a)
}

fn hsl_to_rgb(h: f64, s: f64, l: f64) -> (f64, f64, f64) {
    let r;
    let g;
    let b;

    if s == 0.0 {
        r = l;
        g = l;
        b = l; // achromatic
    } else {
        let q = if l < 0.5 { l * (1. + s) } else { l + s - l * s };

        let p = 2. * l - q;
        r = hue_to_rgb(p, q, h + 1. / 3.);
        g = hue_to_rgb(p, q, h);
        b = hue_to_rgb(p, q, h - 1. / 3.);
    }

    return (r, g, b);
}

fn hue_to_rgb(p: f64, q: f64, t: f64) -> f64 {
    let mut t = t;
    if t < 0. {
        t += 1.
    }
    if t > 1. {
        t -= 1.
    };
    if t < 1. / 6. {
        return p + (q - p) * 6. * t;
    }
    if t < 1. / 2. {
        return q;
    }
    if t < 2. / 3. {
        return p + (q - p) * (2. / 3. - t) * 6.;
    }
    return p;
}
