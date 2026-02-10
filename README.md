# One Good Hour

A 60-minute focus timer TUI paired with a short todo list (max 4 tasks). Write down what you want to accomplish, start the timer, and work through your list.

## Install

```sh
make install
```

This installs `1gh` to `~/.cargo/bin` (ensure it's in your PATH).

Alternatively, build without installing:
```sh
make build  # binary at target/release/1gh
```

## Usage

```sh
1gh
```

Each launch starts a fresh session. No data is persisted to disk.