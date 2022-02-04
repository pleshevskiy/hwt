use crate::components;
use crate::state;
use druid::widget::Flex;
use druid::{LocalizedString, Widget, WidgetExt, WindowDesc};

pub fn create() -> WindowDesc<state::App> {
    return WindowDesc::new(build)
        .title(LocalizedString::new("HWT"))
        .window_size((400.0, 400.0));
}

fn build() -> impl Widget<state::App> {
    Flex::column()
        .with_child(components::break_timer::build("Micro").lens(state::App::micro_break))
        .padding((8.0, 8.0))
}
