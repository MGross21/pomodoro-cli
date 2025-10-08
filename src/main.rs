use clap::Parser;
use crossterm::{execute, terminal::{Clear, ClearType}, cursor::MoveTo};
use indicatif::{ProgressBar, ProgressStyle};
use std::{time::Duration, thread::sleep, io::stdout};
use humantime::parse_duration;

#[derive(Parser)]
#[command(name = "Pomodoro Timer")]
#[command(about = "A simple CLI Pomodoro timer with a progress bar. Let's get productive!")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(disable_version_flag = true)]
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
    /// Number of work/break cycles to run (default: 1)
    #[arg(short, long, default_value_t = 1)]
    cycles: u32,
    /// Print version
    #[arg(short = 'v', long = "version", action = clap::ArgAction::Version)]
    version: bool,
}

fn parse_human_duration(input: &str) -> Result<Duration, String> {
    parse_duration(input)
        .or_else(|_| input.parse::<u64>().map(|num| Duration::from_secs(num * 60)).map_err(|_| ()))
        .map_err(|_| format!("Invalid duration: {}. Use e.g. 25m, 1h, 90s, or just 25 for minutes.", input))
}

fn run_timer(secs: u64, style: &ProgressStyle, finish_message: String, alert: bool) {
    let pb = ProgressBar::new(secs);
    pb.set_style(style.clone());
    pb.enable_steady_tick(Duration::from_millis(100));
    
    for i in 1..=secs {
        sleep(Duration::from_secs(1));
        pb.set_position(i);
    }
    pb.finish_with_message(finish_message);
    if alert {
        print!("\x07");
        let _ = std::io::Write::flush(&mut std::io::stdout());
    }
}

fn parse_secs(input: &str, which: &str) -> u64 {
    parse_human_duration(input).unwrap_or_else(|e| {
        eprintln!("Error parsing {}: {}", which, e);
        std::process::exit(1);
    }).as_secs()
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

    for cycle in 1..=args.cycles {
        if args.cycles > 1 {
            println!("Cycle {}/{}", cycle, args.cycles);
        }
        
        run_timer(
            work_secs,
            &work_style,
            format!("Pomodoro done! Take a {} break.", &args.break_l),
            args.alert,
        );
        
        if cycle < args.cycles {
            run_timer(
                break_secs,
                &break_style,
                "Break finished! Ready for another Pomodoro?".to_string(),
                args.alert,
            );
        }
    }
    
    if args.cycles > 1 {
        println!("All {} cycles completed!", args.cycles);
    }
}