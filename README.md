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

### Quick Start

```bash
curl -sSL https://mgross21.github.io/pomodoro-cli/run | bash -s -- [OPTIONS]
```

### Alias Setup

Add this to your shell config (e.g., `~/.bashrc` or `~/.zshrc`):

```bash
alias pomodoro='curl -sSL https://mgross21.github.io/pomodoro-cli/run | bash -s --'
```

Use the alias:

```bash
pomodoro --work 25 --break 5 --alert
```
