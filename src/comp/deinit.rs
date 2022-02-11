use crate::cmd;
use druid::widget::Controller;
use druid::{Data, Env, Event, EventCtx, LifeCycle, LifeCycleCtx, UpdateCtx, Widget};

#[derive(Default)]
pub struct DeinitController {
    deinit: bool,
}

impl<W, D> Controller<D, W> for DeinitController
where
    D: Data,
    W: Widget<D>,
{
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut D, env: &Env) {
        if !self.deinit {
            match event {
                Event::Command(cmd) if cmd.is(cmd::DEINIT_COMP) => self.deinit = true,
                _ => child.event(ctx, event, data, env),
            }
        }
    }

    fn lifecycle(
        &mut self,
        child: &mut W,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &D,
        env: &Env,
    ) {
        if !self.deinit {
            child.lifecycle(ctx, event, data, env)
        }
    }

    fn update(&mut self, child: &mut W, ctx: &mut UpdateCtx, old_data: &D, data: &D, env: &Env) {
        if !self.deinit {
            child.update(ctx, old_data, data, env)
        }
    }
}
