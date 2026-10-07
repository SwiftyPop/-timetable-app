use crate::model::{Session, TimetableData};
use chrono::{Datelike, Utc};

pub const DEFAULT_UNTIL: &str = "20270228T160000Z";

/// RFC 5545 line folding: max 75 octets per line, continuation begins with a space.
pub fn fold_line(line: &str) -> String {
    if line.len() <= 75 {
        return line.to_string();
    }

    let mut result = String::new();
    let mut current_len = 0;

    for ch in line.chars() {
        let ch_len = ch.len_utf8();
        if current_len + ch_len > 75 {
            result.push_str("\r\n ");
            current_len = 1;
        }
        result.push(ch);
        current_len += ch_len;
    }

    result
}

/// Escape text per RFC 5545: backslashes, semicolons, commas, and newlines.
pub fn escape_ics_text(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 16);
    for ch in s.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            ';' => out.push_str("\\;"),
            ',' => out.push_str("\\,"),
            '\n' => out.push_str("\\n"),
            '\r' => {}
            _ => out.push(ch),
        }
    }
    out
}

pub fn generate_vevent(session: &Session, alarm_minutes: Option<u32>) -> String {
    let by_day = match session.day_index {
        1 => "MO",
        2 => "TU",
        3 => "WE",
        4 => "TH",
        5 => "FR",
        _ => "MO",
    };

    // Calculate next occurrence date for this session
    let now = Utc::now();
    let today = now.date_naive();
    let today_weekday = today.weekday().number_from_monday(); // 1=Mon .. 7=Sun
    let diff = (session.day_index as i64 - today_weekday as i64).rem_euclid(7);
    let target_date = today + chrono::Duration::days(diff);

    let dt_start = format!(
        "{:04}{:02}{:02}T{:02}0000",
        target_date.year(),
        target_date.month(),
        target_date.day(),
        session.start_hour
    );
    let dt_end = format!(
        "{:04}{:02}{:02}T{:02}0000",
        target_date.year(),
        target_date.month(),
        target_date.day(),
        session.end_hour
    );
    let dt_stamp = now.format("%Y%m%dT%H%M%SZ").to_string();
    let uid = format!("class-{}-{}-{}@campustimetable.local", session.id, target_date, session.day_index);

    let summary = format!("{} ({})", session.course_name, session.session_type);
    let instructors = if session.instructors.is_empty() {
        "-".to_string()
    } else {
        session.instructors.join(", ")
    };
    let description = format!(
        "{}\\nVenue: {}\\nLecturers: {}",
        session.course_code, session.venue, instructors
    );

    let mut lines = Vec::new();
    lines.push("BEGIN:VEVENT".to_string());
    lines.push(fold_line(&format!("UID:{}", uid)));
    lines.push(fold_line(&format!("DTSTAMP:{}", dt_stamp)));
    lines.push(fold_line(&format!("SUMMARY:{}", escape_ics_text(&summary))));
    lines.push(fold_line(&format!("DESCRIPTION:{}", description)));
    lines.push(fold_line(&format!("LOCATION:{}", escape_ics_text(&session.venue))));
    lines.push(fold_line(&format!("DTSTART;TZID=Asia/Kuala_Lumpur:{}", dt_start)));
    lines.push(fold_line(&format!("DTEND;TZID=Asia/Kuala_Lumpur:{}", dt_end)));
    lines.push(fold_line(&format!(
        "RRULE:FREQ=WEEKLY;UNTIL={};BYDAY={}",
        DEFAULT_UNTIL, by_day
    )));

    // VALARM for native calendar pop-ups
    if let Some(mins) = alarm_minutes {
        lines.push("BEGIN:VALARM".to_string());
        lines.push("ACTION:DISPLAY".to_string());
        lines.push(fold_line(&format!(
            "DESCRIPTION:Class Reminder: {}",
            escape_ics_text(&session.course_name)
        )));
        lines.push(format!("TRIGGER:-PT{}M", mins));
        lines.push("END:VALARM".to_string());
    }

    lines.push("END:VEVENT".to_string());
    lines.join("\r\n")
}

pub fn generate_calendar_ics(data: &TimetableData, alarm_minutes: Option<u32>) -> String {
    let mut parts = Vec::new();
    parts.push("BEGIN:VCALENDAR".to_string());
    parts.push("VERSION:2.0".to_string());
    parts.push("PRODID:-//Campus Timetable (Unofficial)//EN".to_string());
    parts.push("CALSCALE:GREGORIAN".to_string());
    parts.push("METHOD:PUBLISH".to_string());
    parts.push(
        "BEGIN:VTIMEZONE\r\n\
TZID:Asia/Kuala_Lumpur\r\n\
BEGIN:STANDARD\r\n\
DTSTART:19700101T000000\r\n\
TZOFFSETFROM:+0800\r\n\
TZOFFSETTO:+0800\r\n\
TZNAME:MYT\r\n\
END:STANDARD\r\n\
END:VTIMEZONE"
            .to_string(),
    );

    for session in &data.schedule {
        parts.push(generate_vevent(session, alarm_minutes));
    }

    parts.push("END:VCALENDAR".to_string());
    parts.join("\r\n") + "\r\n"
}
