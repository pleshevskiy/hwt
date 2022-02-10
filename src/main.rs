mod commands;
mod components;
mod delegate;
mod env;
mod state;
mod windows;

use delegate::Delegate;
use druid::AppLauncher;

fn main() {
    let initial_state = state::App {
        paused: false,
        micro_break: state::BreakTimer::new(),
        rest_break: state::BreakTimer::new(),
        notifier: state::Timer::new(),
    };

    AppLauncher::with_window(windows::status::create())
        .use_simple_logger()
        .delegate(Delegate)
        .configure_env(env::configure)
        .launch(initial_state)
        .expect("Failed to launch application");
}
