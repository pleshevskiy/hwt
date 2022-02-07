use crate::commands;
use crate::components;
use crate::state;
use druid::widget::{Flex, Label};
use druid::{Widget, WidgetExt};

pub fn build(name: &str) -> impl Widget<state::BreakTimer> {
    let name_label = Label::new(name);
    Flex::row()
        .with_child(name_label)
        .with_child(components::timer::build().lens(state::BreakTimer::timer))
        .controller(components::timer::CycleTimerController::new(|ctx| {
            ctx.submit_command(commands::OPEN_BREAK_WINDOW.with(ctx.widget_id()));
        }))
}
