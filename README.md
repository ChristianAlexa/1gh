# One Good Hour

All you need is **one good hour**.

A 60-minute focus timer paired with a short todo list (max 4 tasks). Write down what you want to accomplish, start the timer, and work through your list.

![1gh screenshot](docs/1gh-screenshot-tauri.png)

## Install

Each launch starts a fresh session. No data is persisted to disk.

### Desktop App (macOS)

Download the `.dmg` from the [latest release](https://github.com/christianalexa/1gh/releases/latest). The app is unsigned, so run this once after installing:

```
xattr -cr "/Applications/One Good Hour.app"
```

### Terminal

```
curl -fsSL https://onegoodhour.com/install.sh | sh
```

Then just run `1gh`. Requires a terminal with [truecolor](https://github.com/termstandard/colors#truecolor) support (iTerm2, Alacritty, Kitty, WezTerm, etc.). macOS Terminal.app does not support truecolor — use the desktop app instead.

### Build from source

Requires the [Rust toolchain](https://rustup.rs).

```
git clone https://github.com/christianalexa/1gh.git
cd 1gh
make install
1gh
```

To run without installing, use `make run` from the project directory.