use crate::cmd;
use crate::state;
use crate::win;
use druid::{AppDelegate, Command, DelegateCtx, Env, Handled, Target, WindowId};
use log::info;

#[derive(Default)]
pub struct Delegate {
    status_win_id: Option<WindowId>,
}

impl AppDelegate<state::App> for Delegate {
    fn command(
        &mut self,
        ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut state::App,
        _env: &Env,
    ) -> Handled {
        match cmd {
            _ if cmd.is(druid::commands::CLOSE_WINDOW) => match cmd.target() {
                Target::Window(id) if Some(id) == self.status_win_id => {
                    ctx.submit_command(druid::commands::QUIT_APP);
                    Handled::No
                }
                _ => Handled::No,
            },
            _ if cmd.is(cmd::OPEN_NOTIFIER_WINDOW) => {
                let (widget_id, rest_duration_secs) = *cmd.get_unchecked(cmd::OPEN_NOTIFIER_WINDOW);
                ctx.new_window(win::notifier::create(
                    widget_id,
                    rest_duration_secs,
                    data.sound_sender.clone(),
                ));
                Handled::Yes
            }
            _ if cmd.is(cmd::OPEN_IDLE_WINDOW) => {
                let (widget_id, rest_duration_secs) = *cmd.get_unchecked(cmd::OPEN_IDLE_WINDOW);
                ctx.new_window(win::rest::create(
                    widget_id,
                    rest_duration_secs,
                    data.sound_sender.clone(),
                ));
                Handled::Yes
            }
            _ if cmd.is(cmd::PAUSE_ALL_TIMER_COMP) => {
                data.paused = true;
                Handled::No
            }
            _ if cmd.is(cmd::UNPAUSE_ALL_TIMER_COMP) => {
                data.paused = false;
                Handled::No
            }
            _ => Handled::No,
        }
    }

    fn window_added(
        &mut self,
        id: WindowId,
        _data: &mut state::App,
        _env: &Env,
        ctx: &mut DelegateCtx,
    ) {
        info!("Window added, id: {:?}", id);
        if self.status_win_id.is_none() {
            self.status_win_id = Some(id);
        } else {
            ctx.submit_command(
                druid::commands::CONFIGURE_WINDOW
                    .with(druid::WindowConfig::default().set_level(druid_shell::WindowLevel::Modal))
                    .to(Target::Window(id)),
            );
        }
    }
}
