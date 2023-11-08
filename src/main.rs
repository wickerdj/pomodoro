use clap::Parser;
use indicatif::ProgressBar;
use std::io::{Read, Write};
use std::path::Path;
use std::time::Duration;
use std::{fs, thread};

#[derive(Parser)]
struct PomodoroConfig {
    #[clap(short = 't', long = "task")]
    task: String,
    #[clap(short = 'w', long = "work-duration")]
    work_duration: Option<u32>,
    #[clap(short = 'b', long = "break-duration")]
    break_duration: Option<u32>,
    #[clap(short = 'g', long = "global-work")]
    global_work_duration: Option<u32>,
    #[clap(short = 'l', long = "global-break")]
    global_break_duration: Option<u32>,
}

fn main() {
    let mut config = PomodoroConfig::parse();

    // Load global settings from file
    let settings_file = "pomodoro_settings.txt";
    let path = Path::new(settings_file);
    if path.exists() {
        let mut settings_file = fs::File::open(settings_file).unwrap();
        let mut settings_content = String::new();
        settings_file.read_to_string(&mut settings_content).unwrap();

        for line in settings_content.lines() {
            let (key, value) = line.split_once('=').unwrap();
            match key {
                "work_duration" => config.global_work_duration = Some(value.parse().unwrap()),
                "break_duration" => config.global_break_duration = Some(value.parse().unwrap()),
                _ => (),
            }
        }
    }

    // Handle invalid input
    if config.work_duration.is_none() || config.break_duration.is_none() {
        println!("Error: Please specify work and break durations using -w and -b flags.");
        return;
    }

    let work_duration = config
        .work_duration
        .unwrap_or(config.global_work_duration.unwrap_or(25));
    let break_duration = config
        .break_duration
        .unwrap_or(config.global_break_duration.unwrap_or(5));

    // Print welcome message
    println!("Starting Pomodoro session for task: {}", config.task);

    // Run Pomodoro cycles
    loop {
        // Work session
        println!("Work session starting...");

        let progress_bar = ProgressBar::new(work_duration as u64);

        for _minute in 1..=work_duration {
            progress_bar.inc(1);
            thread::sleep(Duration::from_secs(60));
        }

        progress_bar.finish();
        println!("Work session completed.");

        // User interaction
        let mut input = String::new();
        println!("Do you want to take a break? (y/n)");
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim() != "y" {
            continue;
        }

        // Break session
        println!("Taking a break...");

        let progress_bar: ProgressBar = ProgressBar::new(break_duration as u64);

        for _minute in 1..=break_duration {
            progress_bar.inc(1);
            thread::sleep(Duration::from_secs(60));
        }

        progress_bar.finish();
        println!("Break session completed.");

        // User interaction
        println!("Do you want to continue with another work session? (y/n)");
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim() != "y" {
            break;
        }
    }

    // Save global settings to file
    let settings_file = "pomodoro_settings.txt";
    let settings_content = format!(
        "work_duration={}\nbreak_duration={}",
        work_duration, break_duration
    );
    fs::File::create(settings_file)
        .unwrap()
        .write_all(settings_content.as_bytes())
        .unwrap();
}
