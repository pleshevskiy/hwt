use crate::state;
use druid::widget::{Controller, Flex, Label, ProgressBar};
use druid::{Env, Event, EventCtx, TimerToken, Widget, WidgetExt};
use std::time::{Duration, Instant};

const TIMER_INTERVAL: Duration = Duration::from_millis(100);

pub fn build<Cb>(name: &str, on_cicle_cmd: Cb) -> impl Widget<state::BreakTimer>
where
    Cb: Fn(&mut EventCtx) -> (),
{
    let name_label = Label::new(name);
    let time_label =
        Label::dynamic(|data: &String, _: &Env| data.clone()).lens(state::BreakTimer::time);
    let progress_bar = ProgressBar::new().lens(state::BreakTimer::progress);

    Flex::row()
        .with_child(name_label)
        .with_child(time_label)
        .with_child(progress_bar)
        .with_id(if name == "Break" {
            state::NOTIFIER_TIMER
        } else {
            druid::WidgetId::next()
        })
        .controller(BreakTimerController::new(on_cicle_cmd))
}

struct BreakTimerController<Cb>
where
    Cb: Fn(&mut EventCtx) -> (),
{
    start_instant: Instant,
    timer_id: TimerToken,
    cicle_timer_id: TimerToken,
    on_cicle_cmd: Cb,
}

impl<Cb> BreakTimerController<Cb>
where
    Cb: Fn(&mut EventCtx) -> (),
{
    fn new(on_cicle_cmd: Cb) -> Self {
        Self {
            start_instant: Instant::now(),
            timer_id: TimerToken::INVALID,
            cicle_timer_id: TimerToken::INVALID,
            on_cicle_cmd,
        }
    }
}

impl<W, Cb> Controller<state::BreakTimer, W> for BreakTimerController<Cb>
where
    W: Widget<state::BreakTimer>,
    Cb: Fn(&mut EventCtx) -> (),
{
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
                self.start_instant = Instant::now();
                self.timer_id = ctx.request_timer(TIMER_INTERVAL);
                self.cicle_timer_id = ctx.request_timer(Duration::from_secs(data.duration.into()));
                data.progress = data.calculate_progress(Duration::new(0, 0));
                data.time = data.create_time_string(Duration::new(0, 0));

                if ctx.widget_id() == state::NOTIFIER_TIMER {
                    dbg!(&self.start_instant.elapsed().as_secs());
                }
            }
            Event::Timer(id) if *id == self.timer_id => {
                let elapsed = self.start_instant.elapsed();
                data.progress = data.calculate_progress(elapsed);
                data.time = data.create_time_string(elapsed);

                if ctx.widget_id() == state::NOTIFIER_TIMER {
                    dbg!(&elapsed.as_secs(), &data.time, &data.progress);
                }
                // dbg!(&data.time, &elapsed.as_secs());
                ctx.request_paint();

                self.timer_id = ctx.request_timer(TIMER_INTERVAL);
            }
            Event::Timer(id) if *id == self.cicle_timer_id => {
                (self.on_cicle_cmd)(ctx);
            }
            _ => child.event(ctx, event, data, env),
        }
    }
}
