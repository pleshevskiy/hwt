mod cmd;
mod comp;
mod delegate;
mod env;
mod state;
mod win;

use delegate::Delegate;
use druid::AppLauncher;

fn main() {
    let initial_state = state::App {
        paused: false,
        micro_break: state::BreakTimer::new(),
        rest_break: state::BreakTimer::new(),
        notifier: state::Timer::new(),
    };

    AppLauncher::with_window(win::status::create())
        .use_simple_logger()
        .delegate(Delegate)
        .configure_env(env::configure)
        .launch(initial_state)
        .expect("Failed to launch application");
}
