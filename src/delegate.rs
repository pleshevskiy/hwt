use crate::cmd;
use crate::state;
use crate::win;
use druid::{AppDelegate, Command, DelegateCtx, Env, Handled, Target, WindowId};
use log::info;

pub struct Delegate;

impl AppDelegate<state::App> for Delegate {
    fn command(
        &mut self,
        ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        _data: &mut state::App,
        _env: &Env,
    ) -> Handled {
        match cmd {
            _ if cmd.is(cmd::OPEN_NOTIFIER_WINDOW) => {
                let widget_id = *cmd.get_unchecked(cmd::OPEN_NOTIFIER_WINDOW);
                ctx.new_window(win::notifier::create(widget_id));
                Handled::Yes
            }
            _ if cmd.is(cmd::OPEN_IDLE_WINDOW) => {
                let (widget_id, wait_duration) = *cmd.get_unchecked(cmd::OPEN_IDLE_WINDOW);
                ctx.new_window(win::idle::create(widget_id, wait_duration));
                Handled::Yes
            }
            _ => Handled::No,
        }
    }

    fn window_added(
        &mut self,
        id: WindowId,
        _data: &mut state::App,
        _env: &Env,
        _ctx: &mut DelegateCtx,
    ) {
        info!("Window added, id: {:?}", id);
    }
}
