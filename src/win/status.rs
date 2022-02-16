use crate::cmd;
use crate::comp;
use crate::env;
use crate::sound;
use crate::state;
use druid::widget::{Button, Either};
use druid::{LocalizedString, MenuDesc, Widget, WidgetExt, WindowDesc};
use std::rc::Rc;

pub fn create(sender: Rc<sound::Sender>) -> WindowDesc<state::App> {
    let win_width = 220.0;
    let win_height = 100.0;
    WindowDesc::new(|| build(sender))
        .title(LocalizedString::new("HWT Status"))
        .menu(MenuDesc::empty())
        .with_min_size((win_width, win_height))
        .window_size((win_width, win_height))
}

fn build(sender: Rc<sound::Sender>) -> impl Widget<state::App> {
    comp::flex::col_sta_sta()
        .with_child(build_timers(sender))
        .with_default_spacer()
        .with_child(build_pause_btn())
        .padding((8.0, 8.0))
}

fn build_timers(sender: Rc<sound::Sender>) -> impl Widget<state::App> {
    comp::flex::col_sta_sta()
        .with_child(
            comp::break_timer::build(
                "Micro",
                env::MICRO_BREAK_TIMER_DURATION,
                env::MICRO_BREAK_TIMER_POSTPONE_DURATION,
                env::MICRO_BREAK_TIMER_REST_DURATION,
                sender.clone(),
            )
            .lens(state::App::micro_break),
        )
        .with_default_spacer()
        .with_child(
            comp::break_timer::build(
                "Rest",
                env::REST_BREAK_TIMER_DURATION,
                env::REST_BREAK_TIMER_POSTPONE_DURATION,
                env::REST_BREAK_TIMER_REST_DURATION,
                sender.clone(),
            )
            .lens(state::App::rest_break),
        )
}

fn build_pause_btn() -> impl Widget<state::App> {
    Either::new(
        |data: &state::App, _env| data.paused,
        Button::new("Unpause").on_click(|ctx, data: &mut state::App, _env| {
            data.paused = false;
            ctx.submit_command(cmd::UNPAUSE_ALL_TIMER_COMP.with(false))
        }),
        Button::new("Pause").on_click(|ctx, data: &mut state::App, _env| {
            data.paused = true;
            ctx.submit_command(cmd::PAUSE_ALL_TIMER_COMP)
        }),
    )
}
