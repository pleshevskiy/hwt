use crate::state;
use druid::widget::{Controller, Flex, Label, ProgressBar};
use druid::{Env, Event, EventCtx, TimerToken, Widget, WidgetExt};
use std::time::Duration;

const TIMER_INTERVAL: Duration = Duration::from_millis(100);

pub fn build(name: &str) -> impl Widget<state::BreakTimer> {
    let name_label = Label::new(name);
    let time_label =
        Label::new(|data: &String, _: &Env| data.clone()).lens(state::BreakTimer::time);
    let progress_bar = ProgressBar::new().lens(state::BreakTimer::progress);

    Flex::row()
        .with_child(name_label)
        .with_child(time_label)
        .with_child(progress_bar)
        .controller(BreakTimerController::default())
}

struct BreakTimerController {
    timer_id: TimerToken,
}

impl Default for BreakTimerController {
    fn default() -> Self {
        BreakTimerController {
            timer_id: TimerToken::INVALID,
        }
    }
}

impl<W: Widget<state::BreakTimer>> Controller<state::BreakTimer, W> for BreakTimerController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut state::BreakTimer,
        env: &Env,
    ) {
        match event {
            Event::WindowConnected => {
                self.timer_id = ctx.request_timer(TIMER_INTERVAL);
                data.progress = data.calculate_progress();
                data.time = data.create_time_string();
            }
            Event::Timer(id) if *id == self.timer_id => {
                data.progress = data.calculate_progress();
                data.time = data.create_time_string();
                ctx.request_layout();

                self.timer_id = ctx.request_timer(TIMER_INTERVAL);
            }
            _ => child.event(ctx, event, data, env),
        }
    }
}
