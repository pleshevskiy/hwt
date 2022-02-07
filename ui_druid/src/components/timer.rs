use crate::commands;
use crate::state;
use druid::widget::{Controller, Flex, Label, ProgressBar};
use druid::{Env, Event, EventCtx, TimerToken, Widget, WidgetExt};
use std::time::{Duration, Instant};

const TIMER_INTERVAL: Duration = Duration::from_millis(100);

pub fn build() -> impl Widget<state::Timer> {
    let time_label = Label::dynamic(|data: &String, _: &Env| data.clone()).lens(state::Timer::time);
    let progress_bar = ProgressBar::new().lens(state::Timer::progress);

    Flex::row()
        .with_child(time_label)
        .with_child(progress_bar)
        .controller(TimerController::new())
}

struct TimerController {
    current_duration_index: usize,
    start_time: Instant,
    temp_time: Option<Instant>,
    interval_timer_id: TimerToken,
    cycle_timer_id: TimerToken,
}

impl TimerController {
    fn new() -> Self {
        TimerController {
            current_duration_index: Default::default(),
            start_time: Instant::now(),
            temp_time: None,
            interval_timer_id: TimerToken::INVALID,
            cycle_timer_id: TimerToken::INVALID,
        }
    }

    fn current_timer(&self) -> Instant {
        self.temp_time.unwrap_or(self.start_time)
    }

    fn current_duration(&self, data: &state::Timer) -> u32 {
        *data
            .durations
            .iter()
            .nth(self.current_duration_index)
            .unwrap()
    }

    fn next_duration(&mut self, data: &state::Timer) -> u32 {
        let (index, duration) = data
            .durations
            .iter()
            .enumerate()
            .cycle()
            .skip(self.current_duration_index + 1)
            .next()
            .unwrap();
        self.current_duration_index = index;
        *duration
    }
}

impl<W> Controller<state::Timer, W> for TimerController
where
    W: Widget<state::Timer>,
{
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut state::Timer,
        env: &Env,
    ) {
        let current_widget_id = ctx.widget_id();

        match event {
            Event::WindowConnected => {
                let current_duration = self.current_duration(data);

                self.start_time = Instant::now();
                self.interval_timer_id = ctx.request_timer(TIMER_INTERVAL);
                self.cycle_timer_id =
                    ctx.request_timer(Duration::from_secs(current_duration.into()));
                data.progress = calculate_progress(Duration::new(0, 0), current_duration);
                data.time = create_time_string(Duration::new(0, 0), current_duration);
            }
            Event::Timer(id) if *id == self.interval_timer_id => {
                let elapsed = self.current_timer().elapsed();
                let current_duration = self.current_duration(data);
                data.progress = calculate_progress(elapsed, current_duration);
                data.time = create_time_string(elapsed, current_duration);

                ctx.request_paint();

                self.interval_timer_id = ctx.request_timer(TIMER_INTERVAL);
            }
            Event::Timer(id) if *id == self.cycle_timer_id => {
                let current_duration = self.next_duration(data);

                self.temp_time = Some(Instant::now());
                self.cycle_timer_id =
                    ctx.request_timer(Duration::from_secs(current_duration.into()));
                ctx.submit_notification(
                    commands::CYCLE_NOTIFICATION.with(self.current_duration_index),
                )
            }
            Event::Command(cmd) if cmd.is(commands::ACCEPT_TEMP_TIMER) => {
                match cmd.get(commands::ACCEPT_TEMP_TIMER) {
                    Some(widget_id) if widget_id == &current_widget_id => {
                        self.start_time = self.temp_time.take().unwrap_or_else(Instant::now)
                    }
                    _ => child.event(ctx, event, data, env),
                }
            }
            _ => child.event(ctx, event, data, env),
        }
    }
}

fn calculate_progress(elapsed: Duration, duration: u32) -> f64 {
    (elapsed.as_secs_f64() / duration as f64).min(1.0)
}

fn create_time_string(elapsed: Duration, duration: u32) -> String {
    let all_secs = (duration as i64) - (elapsed.as_secs() as i64);
    let sign = if all_secs < 0 { "-" } else { "" };
    let all_secs = all_secs.abs();

    let mins = all_secs / 60;
    let secs = all_secs % 60;
    format!(
        "{}{}:{}{}",
        sign,
        mins,
        if secs < 10 { "0" } else { "" },
        secs
    )
}

pub struct CycleTimerController {
    handler: Box<dyn Fn(&mut EventCtx)>,
}

impl CycleTimerController {
    pub fn new<H>(handler: H) -> Self
    where
        H: Fn(&mut EventCtx) + 'static,
    {
        Self {
            handler: Box::new(handler),
        }
    }
}

impl<W, D> Controller<D, W> for CycleTimerController
where
    W: Widget<D>,
{
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut D, env: &Env) {
        match event {
            Event::Notification(notify) if notify.is(commands::CYCLE_NOTIFICATION) => {
                (self.handler)(ctx);
                ctx.set_handled()
            }
            _ => child.event(ctx, event, data, env),
        }
    }
}
