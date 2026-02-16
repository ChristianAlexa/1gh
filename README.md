# One Good Hour

All you need is **one good hour**.

A 60-minute focus timer TUI paired with a short todo list (max 4 tasks). Write down what you want to accomplish, start the timer, and work through your list.

![1gh screenshot](docs/1gh-screenshot.png)

## Install

Each launch starts a fresh session. No data is persisted to disk.

### macOS (Recommended)

```
curl -fsSL https://onegoodhour.com/install.sh | sh
```

Then just run `1gh` in your terminal.

### Via Cargo

```
cargo install one-good-hour
```

### Build from source

Requires the [Rust toolchain](https://rustup.rs).

```
git clone https://github.com/christianalexa/1gh.git
cd 1gh
make install
1gh
```

To run without installing, use `make run` from the project directory.