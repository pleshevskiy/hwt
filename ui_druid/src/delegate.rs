use crate::commands;
use crate::state;
use crate::windows;
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
        if cmd.is(commands::OPEN_BREAK_WINDOW) {
            let widget_id = *cmd.get_unchecked(commands::OPEN_BREAK_WINDOW);
            ctx.new_window(windows::break_notifier::create(widget_id));
            Handled::Yes
        } else {
            Handled::No
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
