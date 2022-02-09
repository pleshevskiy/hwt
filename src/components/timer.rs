use crate::commands;
use crate::state;
use druid::widget::{Controller, Flex, Label, ProgressBar};
use druid::{Env, Event, EventCtx, TimerToken, Widget, WidgetExt, WidgetId};
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
    start_time: Instant,
    interval_timer_id: TimerToken,
    pause_time: Option<Instant>,
}

impl TimerController {
    fn new() -> Self {
        TimerController {
            start_time: Instant::now(),
            interval_timer_id: TimerToken::INVALID,
            pause_time: None,
        }
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
        match event {
            Event::WindowConnected => {
                self.start_time = Instant::now();
                self.interval_timer_id = ctx.request_timer(TIMER_INTERVAL);
                data.progress = calculate_progress(Duration::new(0, 0), data.duration);
                data.time = create_time_string(Duration::new(0, 0), data.duration);
            }
            Event::Timer(id) if *id == self.interval_timer_id => {
                let elapsed = self.start_time.elapsed();
                data.progress = calculate_progress(elapsed, data.duration);
                data.time = create_time_string(elapsed, data.duration);
                ctx.request_paint();
                self.interval_timer_id = ctx.request_timer(TIMER_INTERVAL);
            }
            Event::Command(cmd) if cmd.is(commands::PAUSE_TIMER_COMPONENT) => {
                self.pause_time = Some(Instant::now());
                self.interval_timer_id = TimerToken::INVALID;
            }
            Event::Command(cmd) if cmd.is(commands::UNPAUSE_TIMER_COMPONENT) => {
                if let Some(instant) = self.pause_time.take() {
                    self.start_time += instant.elapsed();
                    self.interval_timer_id = ctx.request_timer(TIMER_INTERVAL);
                }
            }
            Event::Command(cmd)
                if cmd.is(commands::RESTART_TIMER_COMPONENT)
                    && cmd.get_unchecked(commands::RESTART_TIMER_COMPONENT) == &ctx.widget_id() =>
            {
                self.start_time = Instant::now();
                self.interval_timer_id = ctx.request_timer(TIMER_INTERVAL);
                data.progress = calculate_progress(Duration::new(0, 0), data.duration);
                data.time = create_time_string(Duration::new(0, 0), data.duration);
                ctx.request_paint();
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
    timer_widget_id: WidgetId,
    cycle_timer_id: TimerToken,
}

impl CycleTimerController {
    pub fn new(timer_widget_id: WidgetId) -> Self {
        Self {
            timer_widget_id,
            cycle_timer_id: TimerToken::INVALID,
        }
    }
}

impl<W> Controller<state::Timer, W> for CycleTimerController
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
        match event {
            Event::WindowConnected => {
                self.cycle_timer_id = ctx.request_timer(Duration::from_secs(data.duration.into()));
            }
            Event::Timer(id) if *id == self.cycle_timer_id => {
                self.cycle_timer_id = ctx.request_timer(Duration::from_secs(data.duration.into()));

                ctx.submit_command(commands::RESTART_TIMER_COMPONENT.with(self.timer_widget_id))
            }
            _ => {}
        };
        child.event(ctx, event, data, env);
    }
}
