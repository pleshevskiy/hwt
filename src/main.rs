#![deny(clippy::pedantic)]

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
    let cfg = if let Some(config_path) = std::env::args().nth(1) {
        log::debug!("[cli] config_path={}", config_path);
        let file_content = std::fs::read_to_string(config_path).unwrap_or_else(|err| {
            eprintln!("[ERROR] [fs] {}", err);
            std::process::exit(0);
        });

        toml::from_str(&file_content).unwrap_or_else(|err| {
            eprintln!("[ERROR] [toml] {}", err);
            std::process::exit(0);
        })
    } else {
        log::debug!("use default config");
        config::default()
    };

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
        .configure_env(move |env, _data| env::configure(env, &cfg))
        .launch(initial_state)
        .expect("Failed to launch application");
}
