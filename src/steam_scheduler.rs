use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use chrono::{Local, NaiveTime};
use clokwerk::{Interval, Job, Scheduler};

#[derive(Eq, PartialEq, Copy, Debug, Clone)]
pub struct InvisScheduler {
    pub(crate) interval: Interval,
    pub(crate) start_time: NaiveTime,
    pub(crate) end_time: NaiveTime,
}

impl InvisScheduler {

    pub fn start_schedule(self, stop: &Arc<AtomicBool>) {
        self.update_steam_status();
        let mut scheduler = Scheduler::new();
        let my_stop = Arc::clone(&stop);

        scheduler
            .every(self.interval)
            .at_time(self.start_time)
            .and_every(self.interval)
            .at_time(self.end_time)
            .run(move || self.update_steam_status());

        thread::spawn(move || {
            while !my_stop.load(Ordering::SeqCst) {
                scheduler.run_pending();
                println!("running...");
                thread::sleep(Duration::from_millis(1000));
            }
        });
    }

    fn update_steam_status(&self) {
        let current_time = Local::now().time();

        if current_time >= self.start_time && current_time <= self.end_time {
            self.set_steam_invisible()
        } else {
            self.set_steam_online()
        }
    }

    fn set_steam_online(&self) {
        open::that("steam://friends/status/online").unwrap();
    }

    fn set_steam_invisible(&self) {
        open::that("steam://friends/status/invisible").unwrap();
    }
}
