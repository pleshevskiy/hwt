use crate::commands;
use crate::env;
use crate::state;
use druid::widget::{Controller, Flex, Label, ProgressBar};
use druid::{Env, Event, EventCtx, Key, TimerToken, Widget, WidgetExt};
use std::time::{Duration, Instant};

const TIMER_INTERVAL: Duration = Duration::from_millis(100);

pub fn build() -> impl Widget<state::Timer> {
    let time_label = Label::dynamic(|data: &String, _: &Env| data.clone()).lens(state::Timer::time);
    let progress_bar = ProgressBar::new().lens(state::Timer::progress);

    Flex::row().with_child(time_label).with_child(progress_bar)
}

pub struct TimerController {
    start_time: Instant,
    pause_time: Option<Instant>,
    render_timer_id: TimerToken,
    finish_timer_id: TimerToken,
    finish_handler: Option<Box<dyn Fn(&mut EventCtx)>>,
    postpone_times: u32,
}

impl TimerController {
    pub fn new<Handler>(finish_handler: Handler) -> Self
    where
        Handler: Fn(&mut EventCtx) + 'static,
    {
        Self {
            finish_handler: Some(Box::new(finish_handler)),
            ..Default::default()
        }
    }
}

impl Default for TimerController {
    fn default() -> Self {
        Self {
            start_time: Instant::now(),
            pause_time: None,
            render_timer_id: TimerToken::INVALID,
            finish_timer_id: TimerToken::INVALID,
            finish_handler: Default::default(),
            postpone_times: Default::default(),
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
        let duration = self.duration(env);
        let full_duration = self.full_duration(env);
        match event {
            Event::WindowConnected => {
                self.start_time = Instant::now();
                self.render_timer_id = ctx.request_timer(TIMER_INTERVAL);
                self.finish_timer_id = ctx.request_timer(duration + TIMER_INTERVAL);
                data.reset(duration);
                child.event(ctx, event, data, env);
            }
            Event::Timer(id) if *id == self.render_timer_id => {
                data.update_progress_and_time(self.start_time.elapsed(), duration, full_duration);
                ctx.request_paint();

                self.render_timer_id = ctx.request_timer(TIMER_INTERVAL);
            }
            Event::Timer(id) if *id == self.finish_timer_id => {
                if let Some(finish_handler) = &self.finish_handler {
                    finish_handler(ctx);
                }
            }
            Event::Command(cmd) if cmd.is(commands::PAUSE_ALL_TIMER_COMPONENT) => {
                self.pause_time = Some(Instant::now());
                self.render_timer_id = TimerToken::INVALID;
                self.finish_timer_id = TimerToken::INVALID;
            }
            Event::Command(cmd) if cmd.is(commands::UNPAUSE_ALL_TIMER_COMPONENT) => {
                if let Some(pause_instant) = self.pause_time.take() {
                    self.start_time += pause_instant.elapsed();
                    self.render_timer_id = ctx.request_timer(TIMER_INTERVAL);
                    self.finish_timer_id = ctx.request_timer(
                        duration.saturating_sub(
                            Instant::now().saturating_duration_since(self.start_time),
                        ) + TIMER_INTERVAL,
                    );
                }
            }
            Event::Command(cmd) if cmd.is(commands::RESTART_TIMER_COMPONENT) => {
                self.start_time = Instant::now();
                self.render_timer_id = ctx.request_timer(TIMER_INTERVAL);
                self.finish_timer_id = ctx.request_timer(duration + TIMER_INTERVAL);
                data.reset(duration);
                ctx.request_paint();
            }
            _ => child.event(ctx, event, data, env),
        }
    }
}

impl TimerController {
    fn duration(&self, env: &Env) -> Duration {
        Duration::from_secs_f64(env.get(env::TIMER_DURATION))
    }

    fn postpone_duration(&self, env: &Env) -> Duration {
        match self.postpone_times {
            0 => Duration::ZERO,
            _ => Duration::from_secs_f64(env.get(env::TIMER_POSTPONE_DURATION)),
        }
    }

    fn full_duration(&self, env: &Env) -> Duration {
        self.duration(env) + self.postpone_times * self.postpone_duration(env)
    }
}
