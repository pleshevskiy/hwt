use crate::cmd;
use crate::comp;
use crate::env;
use crate::state;
use druid::widget::{Button, CrossAxisAlignment, Either, Flex};
use druid::{LocalizedString, MenuDesc, Widget, WidgetExt, WindowDesc};

pub fn create() -> WindowDesc<state::App> {
    let win_width = 200.0;
    let win_height = 100.0;
    return WindowDesc::new(build)
        .title(LocalizedString::new("HWT Status"))
        .menu(MenuDesc::empty())
        .with_min_size((win_width, win_height))
        .window_size((win_width, win_height));
}

fn build() -> impl Widget<state::App> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(build_timers())
        .with_default_spacer()
        .with_child(build_pause_btn())
        .padding((8.0, 8.0))
}

fn build_timers() -> impl Widget<state::App> {
    Flex::column()
        .with_child(
            comp::break_timer::build(
                "Micro",
                env::MICRO_BREAK_TIMER_DURATION,
                env::MICRO_BREAK_TIMER_POSTPONE_DURATION,
                env::MICRO_BREAK_TIMER_REST_DURATION,
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
