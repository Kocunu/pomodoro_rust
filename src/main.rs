//Pomodoro Timer

use std::time::{Duration, Instant};
use std::thread;

trait Pomodoro{
    fn start_timer(&mut self);
    fn stop_timer(&self);
    fn count_sessions(&self);
}

pub struct Timer {
    count_timer: Duration,
    is_break: bool,
}

impl Pomodoro for Timer{
    fn start_timer(&mut self) {
        if !self.is_break{
            loop {
                println!("Timer Start");
                let start_time = Instant::now();
                thread::sleep(Duration::from_secs(25));
                let elapsed_time = start_time.elapsed();
                self.count_timer += elapsed_time;
                println!("Timer: {:?}", elapsed_time);
            }
        }        
        else{
            println!("Timer Stopped")
        }
    }

    fn stop_timer(&self) {
        
    }

    fn count_sessions(&self) {
        
    }
}

pub struct user {
    name: String,
    sessions: i32,
}



fn main (){

}
