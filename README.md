# Pomodoro Timer

![Pomodoro Terminal](assets/videos/usage.gif)

## Usage

```console
Usage: pomodoro [OPTIONS]

Options:
  -w, --work <WORK>    Work duration (e.g. 25m, 1h, 90s, or just 25 for minutes) [default: 25]
  -b, --break <BREAK>  Break duration (e.g. 5m, 30s, or just 5 for minutes) [default: 5]
      --alert          Enable alert sound after each timer (if supported by terminal)
  -h, --help           Print help
  -V, --version        Print version
```

## Installation

<sub>Installing Rust package manager, `cargo`: [See Here](https://doc.rust-lang.org/cargo/getting-started/installation.html)</sub>

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
