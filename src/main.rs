use std::time::{Duration, Instant};
use std::{i64, thread,io};
use clap::{App, Arg};
use crossterm::{
    execute, queue,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, ClearType, size},
    cursor,
};
use std::io::Write;

trait Timer {

    fn start_timer(&mut self);
    fn stop_timer(&mut self);
    fn count_sessions(&mut self);
}

pub struct Pomodoro {
    count_timer: Duration,
    is_break: bool,
    duration_minutes: u64,
    is_timer: i64,
}

impl Timer for Pomodoro {
    fn start_timer(&mut self) {
        if !self.is_break {
            // Lösche den Bildschirm
                execute!(
                    io::stdout(),
                    terminal::Clear(ClearType::All),
                    cursor::MoveTo(0, 0)
                )
                .unwrap();

             // Zentriere den Text
                let text = format!(
                    "{}\n{}\n{}\n{}\n{}",
                    "   ___                          _                 ",
                    "  / _ \\___  _ __ ___   ___   __| | ___  _ __ ___  ",
                    " / /_)/ _ \\| '_ ` _ \\ / _ \\ / _` |/ _ \\| '__/ _ \\ ",
                    "/ ___/ (_) | | | | | | (_) | (_| | (_) | | | (_) |",
                    "\\/    \\___/|_| |_| |_|\\___/ \\__,_|\\___/|_|  \\___/ "
                );
                let center = (size().unwrap().0 as usize - text.lines().next().unwrap().len()) / 2;
                queue!(
                    io::stdout(),
                    SetForegroundColor(Color::White),
                    SetBackgroundColor(Color::Black),
                    cursor::MoveTo(center as u16, 0),
                    Print(text+"\n"),
                    ResetColor
                )
                .unwrap();
                io::stdout().flush().unwrap();


            let start_time = Instant::now();
            while !self.is_break {
                thread::sleep(Duration::from_secs(1));
                self.is_timer = self.is_timer + 1;
                println!("{:?}",self.is_timer);
                if start_time.elapsed() >= Duration::from_secs(self.duration_minutes * 60) {
                    self.stop_timer();
                }
            }
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

     execute!(
        io::stdout(),
        terminal::EnterAlternateScreen,
        terminal::Clear(ClearType::All)
    )
    .unwrap();

    let mut _pomodoro = Pomodoro {
        count_timer: Duration::new(0, 0),
        is_break: false,
        duration_minutes: 25, // Standardmäßig 25 Minuten
        is_timer: 0,
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

    let mut pomodoro = Pomodoro {
        count_timer: Duration::new(0, 0),
        is_break: false,
        duration_minutes: matches.value_of("duration").unwrap_or("25").parse().unwrap(),
        is_timer: 0,
    };

    match matches.subcommand_name() {
        Some("start") => pomodoro.start_timer(),
        Some("stop") => pomodoro.stop_timer(),
        Some("count") => pomodoro.count_sessions(),
        _ => println!("Ungültiges Kommando. Verwende 'start', 'stop' oder 'count'."),
    }

    // Beende das Terminal
    execute!(io::stdout(), terminal::LeaveAlternateScreen).unwrap();
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
            is_timer : 0,
        };
        pomodoro.start_timer();
    }

    #[test]
    fn test_stop_timer() {
        let mut pomodoro = Pomodoro {
            count_timer: Duration::new(0, 0),
            is_break: false,
            duration_minutes: 0,
            is_timer : 0,
        };
        pomodoro.start_timer(); 
        pomodoro.stop_timer();
    }
}
