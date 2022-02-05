mod commands;
mod components;
mod delegate;
mod state;
mod uikit;
mod windows;

use delegate::Delegate;
use druid::AppLauncher;

fn main() {
    let initial_state = state::App {
        micro_break: state::BreakTimer {
            duration: 5,
            progress: Default::default(),
            time: Default::default(),
        },
        rest_break: state::BreakTimer {
            duration: 60 * 45,
            progress: Default::default(),
            time: Default::default(),
        },
        notifier: state::BreakTimer {
            duration: 10,
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
