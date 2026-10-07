use tauri::command;
use timetable_core::{
    generate_calendar_ics, validate_timetable, FreeGap, NowInfo, ScheduleEngine, Session,
    SystemClock, TimetableData, ValidationError, WeekStats,
};

fn get_engine() -> ScheduleEngine {
    ScheduleEngine::new(TimetableData::load_default())
}

#[command]
pub fn get_now_next() -> Result<NowInfo, String> {
    let engine = get_engine();
    Ok(engine.now_info(&SystemClock))
}

#[command]
pub fn get_sessions_for_day(day_index: u32) -> Result<Vec<Session>, String> {
    let engine = get_engine();
    Ok(engine.sessions_for_day(day_index))
}

#[command]
pub fn get_free_gaps(day_index: u32) -> Result<Vec<FreeGap>, String> {
    let engine = get_engine();
    Ok(engine.free_gaps_for_day(day_index))
}

#[command]
pub fn get_week_stats() -> Result<WeekStats, String> {
    let engine = get_engine();
    Ok(engine.week_stats())
}

#[command]
pub fn export_calendar_ics(alarm_minutes: Option<u32>) -> Result<String, String> {
    let data = TimetableData::load_default();
    Ok(generate_calendar_ics(&data, alarm_minutes))
}

#[command]
pub fn validate_schedule() -> Result<Vec<ValidationError>, String> {
    let data = TimetableData::load_default();
    Ok(validate_timetable(&data))
}
