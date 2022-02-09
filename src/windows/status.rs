use crate::commands;
use crate::components;
use crate::state;
use druid::widget::{Button, CrossAxisAlignment, Either, Flex};
use druid::{LocalizedString, Widget, WidgetExt, WindowDesc};

pub fn create() -> WindowDesc<state::App> {
    let win_width = 200.0;
    let win_height = 100.0;
    return WindowDesc::new(build)
        .title(LocalizedString::new("HWT Status"))
        .with_min_size((win_width, win_height))
        .window_size((win_width, win_height));
}

fn build() -> impl Widget<state::App> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(components::break_timer::build("Micro").lens(state::App::micro_break))
        .with_default_spacer()
        .with_child(components::break_timer::build("Rest").lens(state::App::rest_break))
        .with_default_spacer()
        .with_child(Either::new(
            |data: &state::App, _env| data.paused,
            Button::new("Unpause").on_click(|ctx, data: &mut state::App, _env| {
                data.paused = false;
                ctx.submit_command(commands::UNPAUSE_ALL_TIMER_COMPONENT)
            }),
            Button::new("Pause").on_click(|ctx, data: &mut state::App, _env| {
                data.paused = true;
                ctx.submit_command(commands::PAUSE_ALL_TIMER_COMPONENT)
            }),
        ))
        .padding((8.0, 8.0))
}
