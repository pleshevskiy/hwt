mod commands;
mod components;
mod delegate;
mod state;
mod windows;

use delegate::Delegate;
use druid::AppLauncher;

fn main() {
    let initial_state = state::App {
        paused: false,
        micro_break: state::BreakTimer::new(5, 30),
        rest_break: state::BreakTimer::new(60 * 45, 30),
        notifier: state::Timer::new(10),
    };

    AppLauncher::with_window(windows::status::create())
        .use_simple_logger()
        .delegate(Delegate)
        .launch(initial_state)
        .expect("Failed to launch application");
}
