mod steam_scheduler;

use chrono::NaiveTime;
use clokwerk::{Interval};
use dioxus::prelude::*;
use crate::steam_scheduler::InvisScheduler;
use std::borrow::Borrow;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use dioxus::html::{div, h1};


fn main() {
    dioxus_desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    let scheduler = InvisScheduler {
        interval: Interval::Monday,
        start_time: NaiveTime::from_hms_opt(5, 0, 0).unwrap(),
        end_time: NaiveTime::from_hms_opt(17, 0, 0).unwrap(),
    };

    let is_start = use_state(cx, || true);
    let stop= use_state(cx, || Arc::new(AtomicBool::default()));

    cx.render(rsx! {
        div {
            display: "flex",
            flex_direction: "column",
            align_content: "center",
            flex_wrap: "wrap",
            h1 {
                "Steam Status Scheduler"
            },
            button {
                width: "auto",
                padding_left: "10px",
                padding_right: "10px",
                align_self: "center",
                onclick: move |_| {
                    if *is_start.get() {
                        stop.get().store(false, Ordering::SeqCst);
                        scheduler.start_schedule(&stop);
                        is_start.set(false);
                    } else {
                        stop.get().store(true, Ordering::SeqCst);
                        is_start.set(true);
                    }
                },
                if *is_start.get() {
                    "Start"
                } else {
                    "Stop"
                },
            },
            if !*is_start.get() {
                running(cx)
            }
        }
    })
}

fn running(cx: Scope) -> Element {
    cx.render (rsx! {
        h3 {
            width: "auto",
            padding_left: "10px",
            padding_right: "10px",
            align_self: "center",
            "Currently running..."
        }
    })
}

