use std::time::{Duration, Instant};
use std::thread;
use clap::{App, Arg};
use indicatif::{ProgressBar, ProgressStyle};

trait Timer {
    fn start_timer(&mut self);
    fn stop_timer(&mut self);
    fn count_sessions(&mut self);
}

pub struct Pomodoro {
    count_timer: Duration,
    is_break: bool,
    duration_minutes: u64,
}

impl Timer for Pomodoro {
    fn start_timer(&mut self) {
        if !self.is_break {
            let pb = ProgressBar::new(self.duration_minutes * 60);
            
            pb.set_style(ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
                .unwrap()
                .progress_chars("##-"));

            let start_time = Instant::now();
            while !self.is_break {
                pb.inc(1);
                thread::sleep(Duration::from_secs(1));
                if start_time.elapsed() >= Duration::from_secs(self.duration_minutes * 60) {
                    self.stop_timer();
                }
            }
            pb.finish_and_clear();
        }
    }

    fn stop_timer(&mut self) {
        self.is_break = true;
        println!("Timer Stopped");
    }

    fn count_sessions(&mut self) {
        println!("Total Elapsed Time: {:?}", self.count_timer);
    }
}

fn main() {
    let mut pomodoro = Pomodoro {
        count_timer: Duration::new(0, 0),
        is_break: false,
        duration_minutes: 25, // Standardmäßig 25 Minuten
    };

    let matches = App::new("Pomodoro CLI")
        .version("1.0")
        .author("Dein Name")
        .about("CLI für den Pomodoro Timer")
        .subcommand(App::new("start").about("Startet den Timer"))
        .subcommand(App::new("stop").about("Stoppt den Timer"))
        .subcommand(App::new("count").about("Zeigt die Gesamtdauer der Sitzungen an"))
        .arg(Arg::with_name("duration")
            .short("d")
            .long("duration")
            .value_name("MINUTES")
            .help("Setzt die Dauer des Pomodoro-Timers in Minuten")
            .takes_value(true))
        .get_matches();

    if let Some(duration_str) = matches.value_of("duration") {
        if let Ok(duration) = duration_str.parse::<u64>() {
            pomodoro.duration_minutes = duration;
        } else {
            eprintln!("Fehler: Die Dauer muss eine positive Ganzzahl sein.");
            return;
        }
    }

    match matches.subcommand_name() {
        Some("start") => pomodoro.start_timer(),
        Some("stop") => pomodoro.stop_timer(),
        Some("count") => pomodoro.count_sessions(),
        _ => println!("Ungültiges Kommando. Verwende 'start', 'stop' oder 'count'."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_timer() {
        let mut pomodoro = Pomodoro {
            count_timer: Duration::new(0, 0),
            is_break: false,
            duration_minutes: 0, // 0 sind 10 Sekunden ?? 
        };
        pomodoro.start_timer();
    }

    #[test]
    fn test_stop_timer() {
        let mut pomodoro = Pomodoro {
            count_timer: Duration::new(0, 0),
            is_break: false,
            duration_minutes: 0,
        };
        pomodoro.start_timer(); 
        pomodoro.stop_timer();
    }
}
