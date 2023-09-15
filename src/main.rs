mod steam_scheduler;

use chrono::NaiveTime;
use clokwerk::Interval;
use dioxus::prelude::*;
use crate::steam_scheduler::InvisScheduler;


fn main() {
    dioxus_desktop::launch(app);
}

fn app(cx: Scope) -> Element {

    let scheduler = InvisScheduler {
        interval: Interval::Monday,
        start_time: NaiveTime::from_hms_opt(5,0,0).unwrap(),
        end_time: NaiveTime::from_hms_opt(17,0,0).unwrap(),
    };

    cx.render(rsx! {
        button {
            onclick: move |_| {
                scheduler.start_schedule()
            },
            "Start"
        }
    })
}

