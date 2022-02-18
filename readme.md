# HWT

**h**ealthy **w**orkaholic **t**imer – A tool that keeps you from breaking your
health by working all day.

## Install

```bash
cargo install hwt_ui
```

## Usage

Type `hwt` in the terminal and the status window of the timers will open with
the default parameters

```sh
hwt
```

If you want to customize the timers, you can create a configuration file based
on [the example](./config.example.toml). Then pass path to your configuration
file as additional parameter.

```sh
hwt ./config.example.toml
```
