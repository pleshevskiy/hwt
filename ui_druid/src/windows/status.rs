use crate::commands;
use crate::components;
use crate::state;
use druid::widget::{CrossAxisAlignment, Flex};
use druid::{LocalizedString, Widget, WidgetExt, WindowDesc};

pub fn create() -> WindowDesc<state::App> {
    return WindowDesc::new(build)
        .title(LocalizedString::new("HWT Status"))
        .window_size((200.0, 100.0));
}

fn build() -> impl Widget<state::App> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(
            components::break_timer::build("Micro", |ctx| {
                ctx.submit_command(commands::OPEN_BREAK_WINDOW.with(ctx.widget_id()))
            })
            .lens(state::App::micro_break),
        )
        .with_default_spacer()
        .with_child(
            components::break_timer::build("Rest", |ctx| {
                ctx.submit_command(commands::OPEN_BREAK_WINDOW.with(ctx.widget_id()))
            })
            .lens(state::App::rest_break),
        )
        .padding((8.0, 8.0))
}
