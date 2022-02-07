use crate::components;
use crate::state;
use druid::widget::{CrossAxisAlignment, Flex};
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
        .padding((8.0, 8.0))
}
