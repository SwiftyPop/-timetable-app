use chrono::{DateTime, FixedOffset, Utc};
use timetable_core::{
    generate_calendar_ics, validate_timetable, Clock, FakeClock, ReminderPlanner, ScheduleEngine,
    SystemClock, TimetableData,
};
use wasm_bindgen::prelude::*;

fn get_engine() -> ScheduleEngine {
    ScheduleEngine::new(TimetableData::load_default())
}

#[wasm_bindgen]
pub fn get_now_next(timestamp_millis: Option<f64>) -> JsValue {
    let engine = get_engine();
    let info = if let Some(ms) = timestamp_millis {
        let secs = (ms / 1000.0) as i64;
        let nsecs = ((ms % 1000.0) * 1_000_000.0) as u32;
        let naive = chrono::DateTime::from_timestamp(secs, nsecs)
            .map(|dt| dt.naive_utc())
            .unwrap_or_else(|| Utc::now().naive_utc());
        let dt = DateTime::<FixedOffset>::from_naive_utc_and_offset(
            naive,
            timetable_core::malaysia_offset(),
        );
        let clock = FakeClock::new(dt);
        engine.now_info(&clock)
    } else {
        engine.now_info(&SystemClock)
    };

    serde_wasm_bindgen::to_value(&info).unwrap_or(JsValue::NULL)
}

#[wasm_bindgen]
pub fn get_sessions_for_day(day_index: u32) -> JsValue {
    let engine = get_engine();
    let sessions = engine.sessions_for_day(day_index);
    serde_wasm_bindgen::to_value(&sessions).unwrap_or(JsValue::NULL)
}

#[wasm_bindgen]
pub fn get_free_gaps(day_index: u32) -> JsValue {
    let engine = get_engine();
    let gaps = engine.free_gaps_for_day(day_index);
    serde_wasm_bindgen::to_value(&gaps).unwrap_or(JsValue::NULL)
}

#[wasm_bindgen]
pub fn get_week_stats() -> JsValue {
    let engine = get_engine();
    let stats = engine.week_stats();
    serde_wasm_bindgen::to_value(&stats).unwrap_or(JsValue::NULL)
}

#[wasm_bindgen]
pub fn compute_reminders(lead_minutes: u32, morning_summary: bool, horizon_days: u32) -> JsValue {
    let data = TimetableData::load_default();
    let planner = ReminderPlanner::new(&data);
    let now = SystemClock.now();
    let reminders = planner.compute_reminders(now, lead_minutes, morning_summary, &[], horizon_days);
    serde_wasm_bindgen::to_value(&reminders).unwrap_or(JsValue::NULL)
}

#[wasm_bindgen]
pub fn export_calendar_ics(alarm_minutes: Option<u32>) -> String {
    let data = TimetableData::load_default();
    generate_calendar_ics(&data, alarm_minutes)
}

#[wasm_bindgen]
pub fn validate_schedule() -> JsValue {
    let data = TimetableData::load_default();
    let errors = validate_timetable(&data);
    serde_wasm_bindgen::to_value(&errors).unwrap_or(JsValue::NULL)
}
