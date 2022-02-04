use crate::state;
use druid::{AppDelegate, DelegateCtx, Env, WindowId};
use log::info;

pub struct Delegate;

impl AppDelegate<state::App> for Delegate {
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
