use crate::cmd;
use crate::comp;
use crate::env;
use crate::state;
use druid::widget::{Flex, Label};
use druid::{Key, Widget, WidgetExt};

pub fn build(
    name: &str,
    duration_env_key: Key<f64>,
    postpone_duration_env_key: Key<f64>,
) -> impl Widget<state::BreakTimer> {
    let name_label = Label::new(name);
    Flex::row().with_child(name_label).with_child(
        comp::timer::build()
            .controller(
                comp::timer::TimerController::new(|ctx| {
                    ctx.submit_command(cmd::PAUSE_ALL_TIMER_COMP);
                    ctx.submit_command(cmd::OPEN_NOTIFIER_WINDOW.with(ctx.widget_id()))
                })
                .with_duration_env(duration_env_key.clone())
                .with_postpone_duration_env(postpone_duration_env_key.clone()),
            )
            .lens(state::BreakTimer::work_timer),
    )
}
