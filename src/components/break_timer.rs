use crate::components;
use crate::state;
use druid::widget::{Flex, Label};
use druid::{Widget, WidgetExt, WidgetId};

pub fn build(name: &str, widget_id: WidgetId) -> impl Widget<state::BreakTimer> {
    let name_label = Label::new(name);
    Flex::row().with_child(name_label).with_child(
        components::timer::build()
            .controller(components::timer::CycleTimerController::new(widget_id))
            .lens(state::BreakTimer::work_timer),
    )
}
