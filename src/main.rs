use clap::Parser;
use crossterm::{execute, terminal::{Clear, ClearType}, cursor::MoveTo};
use indicatif::{ProgressBar, ProgressStyle};
use std::{time::Duration, thread::sleep, io::stdout};
use humantime::parse_duration;

#[derive(Parser)]
#[command(name = "Pomodoro CLI")]
#[command(about = "A simple CLI Pomodoro timer with a progress bar. Let's get productive!")]
#[command(version)]
struct Args {
    /// Work duration (e.g. 25m, 1h, 90s, or just 25 for minutes)
    #[arg(short, long, default_value = "25")]
    work: String,
    /// Break duration (e.g. 5m, 30s, or just 5 for minutes)
    #[arg(short = 'b', long = "break", value_name = "BREAK", default_value = "5")]
    break_l: String, // "break" is a reserved keyword
    /// Enable alert sound after each timer (if supported by terminal)
    #[arg(long, default_value_t = false)]
    alert: bool,
}

fn parse_human_duration(input: &str) -> Result<Duration, String> {
    parse_duration(input)
        .or_else(|_| input.parse::<u64>().map(|num| Duration::from_secs(num * 60)).map_err(|_| ()))
        .map_err(|_| format!("Invalid duration: {}. Use e.g. 25m, 1h, 90s, or just 25 for minutes.", input))
}

fn run_timer(secs: u64, style: &ProgressStyle, finish_message: String, alert: bool) {
    let pb = ProgressBar::new(secs);
    pb.set_style(style.clone());
    for _ in 0..secs {
        pb.inc(1);
        sleep(Duration::from_secs(1));
    }
    pb.finish_with_message(finish_message);
    if alert {
        print!("\x07");
    }
}

fn parse_secs(input: &str, which: &str) -> u64 {
    match parse_human_duration(input) {
        Ok(dur) => dur.as_secs(),
        Err(e) => {
            eprintln!("Error parsing {}: {}", which, e);
            std::process::exit(1);
        }
    }
}

fn main() {
    execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();
    let args = Args::parse();

    let work_secs = parse_secs(&args.work, "work duration");
    let break_secs = parse_secs(&args.break_l, "break duration");

    let work_style = ProgressStyle::with_template(
        "[{elapsed_precise}] [{bar:40.cyan/white}] {percent}%",
    ).unwrap().progress_chars("⣿⣿⣤");
    let break_style = ProgressStyle::with_template(
        "[{elapsed_precise}] [{bar:40.green/white}] {percent}% (Break)",
    ).unwrap().progress_chars("⣿⣿⣤");

    run_timer(
        work_secs,
        &work_style,
        format!("Pomodoro done! Take a {} break.", &args.break_l),
        args.alert,
    );
    run_timer(
        break_secs,
        &break_style,
        "Break finished! Ready for another Pomodoro?".to_string(),
        args.alert,
    );
}