#![windows_subsystem = "windows"]
mod steam_scheduler;

use chrono::NaiveTime;
use clokwerk::{Interval};
use dioxus::prelude::*;
use crate::steam_scheduler::InvisScheduler;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use dioxus_desktop::{Config, WindowBuilder};
use std::str::FromStr;
use clokwerk::Interval::{Weekday};

fn main() {
    dioxus_desktop::launch_cfg(
        app,
        Config::default()
            .with_window(WindowBuilder::default()
                .with_title("Steam Status Scheduler")
            )
    );
}

fn app(cx: Scope) -> Element {
    use_shared_state_provider(cx, || InvisScheduler {
        interval: Interval::Weekday,
        start_time: NaiveTime::from_hms_opt(5, 0, 0).unwrap(),
        end_time: NaiveTime::from_hms_opt(17, 0, 0).unwrap(),
    });
    let scheduler = use_shared_state::<InvisScheduler>(cx).unwrap();
    let is_start = use_state(cx, || true);
    let stop = use_state(cx, || Arc::new(AtomicBool::default()));

    cx.render(rsx! {
        div {
            display: "flex",
            flex_direction: "column",
            align_content: "center",
            flex_wrap: "wrap",
            p {
                flex_direction: "column",
                align_content: "center",
                flex_wrap: "wrap",
                text_align: "center",
                margin_left: "50px",
                margin_right: "50px",
                "Select the interval and time range that you would like to show as invisible in steam"
            }
            interval_input(cx, !is_start.get()),
            button {
                width: "auto",
                padding_left: "10px",
                padding_right: "10px",
                align_self: "center",
                onclick: move |_| {
                    if *is_start.get() {
                        stop.get().store(false, Ordering::SeqCst);
                        scheduler.read().start_schedule(&stop);
                        is_start.set(false);
                    } else {
                        stop.store(true, Ordering::SeqCst);
                        is_start.set(true);
                    }
                },
                if *is_start.get() {
                    "Start"
                } else {
                    "Stop"
                },
            },
            running(cx, is_start.get()),
        }
    })
}

fn interval_input(cx: Scope, disabled: bool) -> Element {
    let scheduler = use_shared_state::<InvisScheduler>(cx).unwrap();
    cx.render(rsx! {
        div {
            margin_bottom:"36px",
            display: "flex",
            flex_direction: "row",
            align_content: "center",
            align_self: "center",
            flex_wrap: "wrap",
            div {
                margin_right:"24px",
                h3 {
                    "Interval"
                }
                input { r#type:"radio", id:"weekday", name:"interval", disabled:disabled, checked:true,
                    onchange: move |_| {
                        let new_scheduler = InvisScheduler{
                            interval: Weekday,
                            start_time: scheduler.read().start_time.clone(),
                            end_time: scheduler.read().start_time.clone(),
                        };
                        *scheduler.write() = new_scheduler;
                    }
                }
                label { r#for:"weekday", "Weekday" } br {} br {}
                input { r#type:"radio", id:"everyday", name:"interval", disabled: disabled,
                    onchange: move |_| {
                        let new_scheduler = InvisScheduler{
                            interval: Interval::Days(1),
                            start_time: scheduler.read().start_time.clone(),
                            end_time: scheduler.read().start_time.clone(),
                        };
                        *scheduler.write() = new_scheduler;
                    }
                }
                label { r#for:"everyday", "Everyday" }
            }
            div {
                margin_left:"24px",
                display: "flex",
                flex_direction: "column",
                align_content: "center",
                flex_wrap: "wrap",
                h3 {
                    "Range"
                }
                label { r#for:"start_time", "Start time" }
                input { r#type:"time", id:"start_time", initial_value:"05:00:00", disabled: disabled,
                    onchange: move |evt| {
                        let new_scheduler = InvisScheduler {
                            interval: scheduler.read().interval.clone(),
                            start_time: NaiveTime::from_str(&evt.value).unwrap(),
                            end_time: scheduler.read().end_time.clone(),
                        };
                        *scheduler.write() = new_scheduler;
                    }
                }
                br{}
                label { r#for:"end_time", "End time" }
                input { r#type:"time", id:"end_time", initial_value:"17:00:00", disabled: disabled,
                    onchange: move |evt| {
                        let new_scheduler = InvisScheduler {
                            interval: scheduler.read().interval.clone(),
                            start_time: scheduler.read().start_time.clone(),
                            end_time: NaiveTime::from_str(&evt.value).unwrap(),
                        };
                        *scheduler.write() = new_scheduler;
                    },
                }
            }
        }

    })
}

fn running<'a>(cx: Scope<'a>, is_start: &'a bool) -> Element<'a> {
    cx.render (rsx! {
        h3 {
            width: "auto",
            padding_left: "10px",
            padding_right: "10px",
            align_self: "center",
            if *is_start {
                "Not running."
            } else {
                "Currently running..."
            }
        }
    })
}

