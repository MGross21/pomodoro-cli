use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;
use std::thread::sleep;

#[derive(Parser)]
#[command(name = "Pomodoro CLI")]
#[command(about = "A simple CLI Pomodoro timer with a progress bar. Let's get productive!")]
struct Args {
    /// Work duration in minutes
    #[arg(short, long, default_value_t = 25)]
    work: u64,
    /// Break duration in minutes
    #[arg(short = 'b', long = "break", value_name = "BREAK", default_value_t = 5)]
    break_l: u64, // "break" is a reserved keyword
    /// Enable alert sound after each timer (if supported by terminal)
    #[arg(long, default_value_t = false)]
    alert: bool,
}

fn disp_progress_bar(total_secs: u64, style: ProgressStyle, finish_message: String) {
    let pb = ProgressBar::new(total_secs);
    pb.set_style(style);
    for _ in 0..total_secs {
        pb.inc(1);
        sleep(Duration::from_secs(1));
    }
    pb.finish_with_message(finish_message);
}

fn main() {
    let args = Args::parse();
    let total_secs = args.work * 60;
    let work_style = ProgressStyle::with_template(
        "[{elapsed_precise}] [{bar:40.cyan/blue}] {percent}%",
    ).unwrap()
    .progress_chars("##-");
    disp_progress_bar(
        total_secs,
        work_style,
        format!("Pomodoro done! 🎉 Take a {} minute break.", args.break_l),
    );

    if args.alert {
        print!("\x07"); // Trigger a sound alert (if supported by terminal)
    }

    let break_secs = args.break_l * 60;
    let break_style = ProgressStyle::with_template(
        "[{elapsed_precise}] [{bar:40.green/white}] {percent}% (Break)",
    ).unwrap()
    .progress_chars("##-");
    disp_progress_bar(
        break_secs,
        break_style,
        "Break finished! Ready for another Pomodoro?".to_string(),
    );

    if args.alert {
        print!("\x07"); // Trigger a sound alert (if supported by terminal)
    }
}