use std::{thread};
use std::time::{Duration};
use chrono::{Local, NaiveTime};
use clokwerk::{Job, Scheduler};
use clokwerk::Interval::Weekday;

fn main() {
    update_steam_status();
    schedule();
    loop_main();
}

fn update_steam_status() {
    let time_range_start = NaiveTime::from_hms_opt(5,0,0).unwrap();
    let time_range_end = NaiveTime::from_hms_opt(17,0,0).unwrap();
    let current_time = Local::now().time();

    if current_time > time_range_start && current_time < time_range_end {
        set_steam_invisible()
    } else {
        set_steam_online()
    }
}

fn set_steam_online() {
    open::that("steam://friends/status/online").unwrap();
}

fn set_steam_invisible() {
    open::that("steam://friends/status/invisible").unwrap();
}

fn loop_main() {
    loop {
        thread::sleep(Duration::from_millis(100));
    }
}

fn schedule() {
    let mut scheduler = Scheduler::new();
    scheduler
        .every(Weekday)
        .at_time(NaiveTime::from_hms_opt(5, 0, 1).unwrap())
        .and_every(Weekday)
        .at_time(NaiveTime::from_hms_opt(17, 0, 1).unwrap())
        .run(move || update_steam_status());

    thread::spawn(move || {
        loop {
            scheduler.run_pending();
            thread::sleep(Duration::from_millis(1000));
        }
    });
}