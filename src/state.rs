use chrono::{Datelike, Local, Timelike};
use crate::models::{ClassEvent, EVENTS};

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AppView {
    Day,
    Week,
}

#[derive(Clone, Debug)]
pub struct NowInfo {
    pub day_of_week: u32,     // 1 = Mon .. 5 = Fri, 0/6 = Weekend
    pub minute_of_day: u32,   // 0..1440
    pub time_str: String,     // "14:30"
    pub date_str: String,     // "Sunday, 4 Oct"
    pub current_class: Option<&'static ClassEvent>,
    pub current_progress: f32, // 0.0 .. 1.0
    pub current_remaining_min: u32,
    pub next_class: Option<&'static ClassEvent>,
    pub next_starts_in_min: u32,
    pub next_day_offset: u32,
}

pub fn calculate_now_info() -> NowInfo {
    let now = Local::now();
    let d = now.weekday().number_from_monday(); // 1 = Mon .. 7 = Sun
    let day_of_week = if d <= 5 { d } else { 0 };
    let m = now.hour() * 60 + now.minute();

    let mut current_class = None;
    let mut current_progress = 0.0;
    let mut current_remaining_min = 0;

    if day_of_week >= 1 && day_of_week <= 5 {
        for ev in EVENTS {
            if ev.day == day_of_week as u8 && m >= (ev.start as u32 * 60) && m < (ev.end as u32 * 60) {
                current_class = Some(ev);
                let total = (ev.end - ev.start) as f32 * 60.0;
                let elapsed = m as f32 - (ev.start as f32 * 60.0);
                current_progress = (elapsed / total).clamp(0.0, 1.0);
                current_remaining_min = (ev.end as u32 * 60).saturating_sub(m);
                break;
            }
        }
    }

    // Find next upcoming class
    let mut next_class = None;
    let mut next_starts_in_min = 0;
    let mut next_day_offset = 0;

    for offset in 0..=7 {
        let check_day = ((d - 1 + offset) % 7) + 1;
        if check_day >= 1 && check_day <= 5 {
            let mut candidates: Vec<&'static ClassEvent> = EVENTS
                .iter()
                .filter(|ev| {
                    ev.day == check_day as u8 && (offset > 0 || (ev.start as u32 * 60) > m)
                })
                .collect();
            candidates.sort_by_key(|e| e.start);
            if let Some(&first) = candidates.first() {
                next_class = Some(first);
                next_day_offset = offset;
                next_starts_in_min = offset * 1440 + (first.start as u32 * 60).saturating_sub(m);
                break;
            }
        }
    }

    NowInfo {
        day_of_week,
        minute_of_day: m,
        time_str: format!("{:02}:{:02}", now.hour(), now.minute()),
        date_str: now.format("%A, %e %b %Y").to_string(),
        current_class,
        current_progress,
        current_remaining_min,
        next_class,
        next_starts_in_min,
        next_day_offset,
    }
}

pub fn format_duration_min(m: u32) -> String {
    if m < 60 {
        format!("{} min", m)
    } else {
        let h = m / 60;
        let rem = m % 60;
        if rem == 0 {
            format!("{} h", h)
        } else {
            format!("{} h {} min", h, rem)
        }
    }
}
