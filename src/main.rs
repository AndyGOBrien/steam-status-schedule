#![windows_subsystem = "windows"]
mod steam_scheduler;

use chrono::NaiveTime;
use clokwerk::{Interval};
use dioxus::prelude::*;
use crate::steam_scheduler::InvisScheduler;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use dioxus::html::{br, form, input, label};
use dioxus_desktop::{Config, WindowBuilder};

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
    let scheduler = use_state(cx, || InvisScheduler {
        interval: Interval::Weekday,
        start_time: NaiveTime::from_hms_opt(5, 0, 0).unwrap(),
        end_time: NaiveTime::from_hms_opt(17, 0, 0).unwrap(),
    });
    let is_start = use_state(cx, || true);
    let stop= use_state(cx, || Arc::new(AtomicBool::default()));

    cx.render(rsx! {
        div {
            display: "flex",
            flex_direction: "column",
            align_content: "center",
            flex_wrap: "wrap",
            interval_input(cx, !is_start.get()),
            button {
                width: "auto",
                padding_left: "10px",
                padding_right: "10px",
                align_self: "center",
                onclick: move |_| {
                    if *is_start.get() {
                        stop.get().store(false, Ordering::SeqCst);
                        scheduler.get().start_schedule(&stop);
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
            running(cx, is_start.get())
        }
    })
}

fn interval_input(cx: Scope, disabled: bool) -> Element {
    cx.render(rsx! {
        form {
            div {
                margin_bottom:"36px",
                display: "flex",
                flex_direction: "row",
                align_content: "center",
                flex_wrap: "wrap",
                div {
                    margin_right:"24px",
                    h3 {
                        "Interval"
                    }
                    input { r#type:"radio", id:"weekday", name:"interval", value:"weekday", disabled: disabled }
                    label { r#for:"weekday", "Weekday" } br {} br {}
                    input { r#type:"radio", id:"weekend", name:"interval", value:"weekend", disabled: disabled }
                    label { r#for:"weekend", "Weekend" } br {} br {}
                    input { r#type:"radio", id:"every-day", name:"interval", value:"every-day", disabled: disabled }
                    label { r#for:"every-day", "Every day" }
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
                    input { r#type:"time", id:"start_time", disabled: disabled }
                    br{}
                    label { r#for:"end_time", "End time" }
                    input { r#type:"time", id:"end_time", disabled: disabled }
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

