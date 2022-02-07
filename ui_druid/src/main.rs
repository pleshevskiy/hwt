mod commands;
mod components;
mod delegate;
mod state;
mod windows;

use delegate::Delegate;
use druid::AppLauncher;

fn main() {
    let initial_state = state::App {
        micro_break: state::BreakTimer {
            timer: state::Timer::multiple(vec![60 * 5, 30]),
        },
        rest_break: state::BreakTimer {
            timer: state::Timer::single(60 * 45),
        },
        notifier: state::Timer::single(10),
    };

    AppLauncher::with_window(windows::status::create())
        .use_simple_logger()
        .delegate(Delegate)
        .launch(initial_state)
        .expect("Failed to launch application");
}
