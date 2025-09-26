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
curl -sSL mgross21.github.io/pomodoro-cli/run|sh -s -- [OPTIONS]
```

### Remote Alias Setup

Add this to your shell config (e.g., `~/.bashrc` or `~/.zshrc`):

```bash
alias pomodoro='curl -sSL mgross21.github.io/pomodoro-cli/run|sh -s --'
```

Use the alias:

```bash
pomodoro --work 25 --break 5 --alert
```

### Download

<a href="https://mgross21.github.io/pomodoro-cli/pomodoro">
  <button style="padding: 4px 10px; font-size: 0.9em; background-color: #9c6ade; color: #fff; border: none; border-radius: 4px; cursor: pointer;">
    Pomodoro Binary
  </button>
</a>
