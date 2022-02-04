mod components;
mod delegate;
mod state;
mod uikit;
mod windows;

use delegate::Delegate;
use druid::AppLauncher;
use std::rc::Rc;
use std::time::Instant;

fn main() {
    let initial_state = state::App {
        micro_break: state::BreakTimer {
            start_instant: Rc::new(Instant::now()),
            duration: 5,
            progress: Default::default(),
            time: Default::default(),
        },
    };

    AppLauncher::with_window(windows::status::create())
        .use_simple_logger()
        .delegate(Delegate)
        .launch(initial_state)
        .expect("Failed to launch application");
}
