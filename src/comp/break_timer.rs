use crate::cmd;
use crate::comp;
use crate::sound;
use crate::state;
use druid::widget::Label;
use druid::{Key, Widget, WidgetExt};
use std::rc::Rc;

pub fn build(
    name: &str,
    duration_env_key: Key<f64>,
    postpone_duration_env_key: Key<f64>,
    rest_duration_env_key: Key<f64>,
    sound_sender: Rc<sound::Sender>,
) -> impl Widget<state::BreakTimer> {
    comp::flex::row_sta_sta()
        .with_child(Label::new(name).align_right().fix_width(50.0))
        .with_child(
            comp::timer::build()
                .controller(
                    comp::timer::TimerController::new(move |ctx, _env, rest_duration_secs| {
                        sound_sender.send(sound::Type::EndBreakTimer).ok();

                        ctx.submit_command(cmd::PAUSE_ALL_TIMER_COMP);
                        ctx.submit_command(
                            cmd::OPEN_NOTIFIER_WINDOW.with((ctx.widget_id(), rest_duration_secs)),
                        );
                    })
                    .with_duration(duration_env_key.clone())
                    .with_postpone_duration(postpone_duration_env_key.clone())
                    .with_rest_duration_env(rest_duration_env_key.clone()),
                )
                .lens(state::BreakTimer::work_timer),
        )
}
