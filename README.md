# Pomodoro Timer

![Pomodoro Terminal](assets/videos/usage.gif)

## Usage

```bash
Usage: pomodoro [OPTIONS]

Options:
    -w, --work  <WORK>   Work duration in minutes [default: 25]
    -b, --break <BREAK>  Break duration in minutes [default: 5]
        --alert          Enable alert sound after each timer (if supported)
    -h, --help           Print help
    -V, --version        Print version
```

## Installation

Clone the repository and build manually:

```bash
git clone https://github.com/MGross21/pomodoro-cli
cd pomodoro-cli
cargo build --release
```

The compiled binary will be at:

```bash
./target/release/pomodoro-cli
```

Or install directly with Cargo:

```bash
cargo install --path .
```

The installed binary will be in:

```bash
~/.cargo/bin/
```

*Ensure this directory is in your `$PATH`.*
