use crate::commands;
use crate::components;
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
        components::timer::build()
            .controller(components::timer::TimerController::new(|ctx| {
                ctx.submit_command(commands::PAUSE_ALL_TIMER_COMPONENT);
                ctx.submit_command(commands::OPEN_BREAK_WINDOW.with(ctx.widget_id()))
            }))
            .env_scope(move |env, _| {
                env.set(env::TIMER_DURATION, env.get(duration_env_key.clone()));
                env.set(
                    env::TIMER_POSTPONE_DURATION,
                    env.get(postpone_duration_env_key.clone()),
                );
            })
            .lens(state::BreakTimer::work_timer),
    )
}
