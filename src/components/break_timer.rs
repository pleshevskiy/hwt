use crate::components;
use crate::state;
use druid::widget::{Flex, Label};
use druid::{Widget, WidgetExt};

pub fn build(name: &str) -> impl Widget<state::BreakTimer> {
    let name_label = Label::new(name);
    Flex::row().with_child(name_label).with_child(
        components::timer::build()
            .controller(components::timer::CycleTimerController::new())
            .lens(state::BreakTimer::work_timer),
    )
}
