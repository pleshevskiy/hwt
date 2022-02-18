mod cmd;
mod comp;
mod config;
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

    let initial_state = state::App::new(tx, true);
    AppLauncher::with_window(win::status::create(initial_state.sound_sender.clone()))
        .delegate(Delegate::default())
        .configure_env(env::configure)
        .launch(initial_state)
        .expect("Failed to launch application");
}
