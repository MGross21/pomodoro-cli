use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;
use std::thread::sleep;

#[derive(Parser)]
#[command(name = "Pomodoro CLI")]
#[command(about = "A simple CLI Pomodoro timer with progress bar")]
struct Args {
    /// Work duration in minutes
    #[arg(short, long, default_value_t = 25)]
    work: u64,
}

fn main() {
    let args = Args::parse();
    let total_secs = args.work * 60;

    let pb = ProgressBar::new(total_secs);
    pb.set_style(ProgressStyle::with_template(
        "[{elapsed_precise}] [{bar:40.cyan/blue}] {percent}%",
    ).unwrap()
    .progress_chars("##-"));

    for _ in 0..total_secs {
        pb.inc(1);
        sleep(Duration::from_secs(1));
    }

    pb.finish_with_message("Pomodoro done! 🎉 Take a break.");
}