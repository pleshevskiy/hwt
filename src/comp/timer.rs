use crate::cmd;
use crate::comp;
use crate::env;
use crate::state;
use druid::widget::{Controller, Label, ProgressBar};
use druid::{Env, Event, EventCtx, KeyOrValue, TimerToken, Widget, WidgetExt};
use std::time::{Duration, Instant};

const TIMER_INTERVAL: Duration = Duration::from_millis(50);

pub fn build() -> impl Widget<state::Timer> {
    let time_label = Label::dynamic(|data: &String, _: &Env| data.clone()).lens(state::Timer::time);
    let progress_bar = ProgressBar::new().lens(state::Timer::progress);

    comp::flex::row_sta_sta()
        .with_child(time_label.align_right().fix_width(55.0))
        .with_child(progress_bar)
}

pub struct TimerController {
    duration: KeyOrValue<f64>,
    init_duration: Option<KeyOrValue<f64>>,
    postpone_duration: Option<KeyOrValue<f64>>,
    rest_duration: Option<KeyOrValue<f64>>,
    start_time: Instant,
    pause_time: Option<Instant>,
    render_timer_id: TimerToken,
    finish_timer_id: TimerToken,
    finish_handler: Option<Box<dyn Fn(&mut EventCtx, f64)>>,
    postpone_times: u32,
}

impl TimerController {
    pub fn new<Handler>(finish_handler: Handler) -> Self
    where
        Handler: Fn(&mut EventCtx, f64) + 'static,
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
            duration: env::TIMER_DURATION.into(),
            init_duration: None,
            postpone_duration: None,
            rest_duration: None,
            start_time: Instant::now(),
            pause_time: None,
            render_timer_id: TimerToken::INVALID,
            finish_timer_id: TimerToken::INVALID,
            finish_handler: None,
            postpone_times: 0,
        }
    }
}

impl TimerController {
    pub fn with_duration(mut self, secs: impl Into<KeyOrValue<f64>>) -> Self {
        self.duration = secs.into();
        self
    }

    pub fn with_init_duration(mut self, secs: impl Into<KeyOrValue<f64>>) -> Self {
        self.init_duration = Some(secs.into());
        self
    }

    pub fn with_postpone_duration(mut self, secs: impl Into<KeyOrValue<f64>>) -> Self {
        self.postpone_duration = Some(secs.into());
        self
    }

    pub fn with_rest_duration_env(mut self, secs: impl Into<KeyOrValue<f64>>) -> Self {
        self.rest_duration = Some(secs.into());
        self
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
                let shift_start_time = Duration::from_secs_f64(
                    self.init_duration
                        .as_ref()
                        .map(|d| d.resolve(env))
                        .unwrap_or_default(),
                );

                self.start_time = Instant::now() - shift_start_time;
                self.render_timer_id = ctx.request_timer(TIMER_INTERVAL);
                self.finish_timer_id = ctx.request_timer(duration - shift_start_time);
                data.reset(duration);
                child.event(ctx, event, data, env);
            }
            Event::Timer(id) if *id == self.render_timer_id => {
                data.update_progress_and_time(self.start_time.elapsed(), full_duration);
                ctx.request_paint();

                self.render_timer_id = ctx.request_timer(TIMER_INTERVAL);
            }
            Event::Timer(id) if *id == self.finish_timer_id => {
                if let Some(finish_handler) = &self.finish_handler {
                    finish_handler(ctx, self.full_rest_duration(env).as_secs_f64());
                }
            }
            Event::Command(cmd) if cmd.is(cmd::PAUSE_ALL_TIMER_COMP) => {
                self.pause_time = Some(Instant::now());
                self.render_timer_id = TimerToken::INVALID;
                self.finish_timer_id = TimerToken::INVALID;
            }
            Event::Command(cmd) if cmd.is(cmd::UNPAUSE_ALL_TIMER_COMP) => {
                let skip_pause = cmd.get_unchecked(cmd::UNPAUSE_ALL_TIMER_COMP);
                self.finish_timer_id =
                    if let (false, Some(pause_instant)) = (skip_pause, self.pause_time.take()) {
                        self.start_time += pause_instant.elapsed();
                        ctx.request_timer(duration.saturating_sub(
                            Instant::now().saturating_duration_since(self.start_time),
                        ))
                    } else if self.postpone_times > 0 {
                        let postpone_duration = self.postpone_duration(env);
                        ctx.request_timer(postpone_duration.saturating_sub(
                            Instant::now().saturating_duration_since(
                                self.start_time + full_duration - postpone_duration,
                            ),
                        ))
                    } else {
                        ctx.request_timer(self.duration(env))
                    };

                self.render_timer_id = ctx.request_timer(TIMER_INTERVAL);
            }
            Event::Command(cmd) if cmd.is(cmd::RESTART_TIMER_COMP) => {
                self.start_time = Instant::now();
                self.postpone_times = 0;
                self.render_timer_id = ctx.request_timer(TIMER_INTERVAL);
                self.finish_timer_id = ctx.request_timer(duration);
                data.reset(duration);
                ctx.request_paint();
            }
            Event::Command(cmd) if cmd.is(cmd::POSTPONE_TIMER_COMP) => {
                self.postpone_times += 1;
            }
            _ => child.event(ctx, event, data, env),
        }
    }
}

impl TimerController {
    fn full_rest_duration(&self, env: &Env) -> Duration {
        self.rest_duration(env) + self.postpone_times * self.postpone_rest_duration(env)
    }

    fn postpone_rest_duration(&self, env: &Env) -> Duration {
        match (&self.postpone_duration, &self.rest_duration) {
            (Some(_), Some(_)) => {
                let duration = self.duration(env).as_secs_f64();
                let rest_duration = self.rest_duration(env).as_secs_f64();
                let postpone_duration = self.postpone_duration(env).as_secs_f64();

                let rest_per_sec = rest_duration / duration;
                Duration::from_secs_f64(postpone_duration * rest_per_sec)
            }
            _ => Duration::ZERO,
        }
    }

    fn rest_duration(&self, env: &Env) -> Duration {
        match &self.rest_duration {
            None => Duration::ZERO,
            Some(d) => Duration::from_secs_f64(d.resolve(env)),
        }
    }

    fn full_duration(&self, env: &Env) -> Duration {
        self.duration(env) + self.postpone_times * self.postpone_duration(env)
    }

    fn duration(&self, env: &Env) -> Duration {
        Duration::from_secs_f64(self.duration.resolve(env))
    }

    fn postpone_duration(&self, env: &Env) -> Duration {
        match self.postpone_times {
            0 => Duration::ZERO,
            _ => {
                let d = self.postpone_duration.as_ref().unwrap_or(&self.duration);
                Duration::from_secs_f64(d.resolve(env))
            }
        }
    }
}
