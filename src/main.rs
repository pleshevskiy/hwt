mod cmd;
mod comp;
mod delegate;
mod env;
mod sound;
mod state;
mod win;

use delegate::Delegate;
use druid::AppLauncher;

use std::sync::mpsc::channel;
use std::thread;

fn main() {
    let (tx, rx) = channel::<sound::Type>();

    thread::spawn(move || loop {
        rx.recv()
            .map_err(From::from)
            .and_then(|sound_type| sound::try_play(sound_type.into()))
            .ok();
    });

    let initial_state = state::App {
        paused: false,
        micro_break: state::BreakTimer::new(),
        rest_break: state::BreakTimer::new(),
        notifier: state::Timer::new(),
        sound_sender: std::rc::Rc::new(tx.clone()),
    };

    AppLauncher::with_window(win::status::create(initial_state.sound_sender.clone()))
        .delegate(Delegate)
        .configure_env(env::configure)
        .launch(initial_state)
        .expect("Failed to launch application");
}
