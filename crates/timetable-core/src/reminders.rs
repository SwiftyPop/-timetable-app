use crate::model::TimetableData;
use crate::time::malaysia_offset;
use chrono::{DateTime, Datelike, Duration, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Reminder {
    pub id: String,
    pub session_id: Option<String>,
    pub title: String,
    pub body: String,
    pub fire_time_iso: String,
    pub fire_timestamp_secs: i64,
    pub is_morning_summary: bool,
}

pub struct ReminderPlanner<'a> {
    data: &'a TimetableData,
}

impl<'a> ReminderPlanner<'a> {
    pub fn new(data: &'a TimetableData) -> Self {
        Self { data }
    }

    /// Compute scheduled reminders for the next `horizon_days` days starting from `now`.
    pub fn compute_reminders(
        &self,
        now: DateTime<FixedOffset>,
        lead_minutes: u32,
        morning_summary: bool,
        muted_codes: &[String],
        horizon_days: u32,
    ) -> Vec<Reminder> {
        let mut reminders = Vec::new();
        let tz = malaysia_offset();

        for day_offset in 0..horizon_days {
            let target_dt = now + Duration::days(day_offset as i64);
            let target_date = target_dt.date_naive();

            // JS day: Sun=0, Mon=1, ..., Sat=6
            let js_day = match target_dt.weekday() {
                chrono::Weekday::Sun => 0,
                chrono::Weekday::Mon => 1,
                chrono::Weekday::Tue => 2,
                chrono::Weekday::Wed => 3,
                chrono::Weekday::Thu => 4,
                chrono::Weekday::Fri => 5,
                chrono::Weekday::Sat => 6,
            };

            if !(1..=5).contains(&js_day) {
                continue;
            }

            let mut day_sessions: Vec<_> = self
                .data
                .schedule
                .iter()
                .filter(|s| s.day_index == js_day)
                .collect();
            day_sessions.sort_by_key(|s| s.start_hour);

            if day_sessions.is_empty() {
                continue;
            }

            // Morning summary at 07:30
            if morning_summary {
                if let Some(morning_naive) = target_date.and_hms_opt(7, 30, 0) {
                    let morning_dt = DateTime::<FixedOffset>::from_naive_utc_and_offset(
                        morning_naive - Duration::hours(8),
                        tz,
                    );

                    if morning_dt > now {
                        let class_count = day_sessions.len();
                        let first = &day_sessions[0];
                        let title = format!("☀️ Today's Classes · {}", first.day_of_week);
                        let body = format!(
                            "{} classes today. First class: {} at {} ({})",
                            class_count, first.course_name, first.start_time, first.room_short
                        );

                        reminders.push(Reminder {
                            id: format!("summary-{}-{}", target_date, js_day),
                            session_id: None,
                            title,
                            body,
                            fire_time_iso: morning_dt.to_rfc3339(),
                            fire_timestamp_secs: morning_dt.timestamp(),
                            is_morning_summary: true,
                        });
                    }
                }
            }

            // Per-class reminders
            for session in &day_sessions {
                // Check if subject is muted
                if muted_codes
                    .iter()
                    .any(|code| session.course_code.contains(code))
                {
                    continue;
                }

                if let Some(start_naive) = target_date.and_hms_opt(session.start_hour, 0, 0) {
                    let class_start_dt = DateTime::<FixedOffset>::from_naive_utc_and_offset(
                        start_naive - Duration::hours(8),
                        tz,
                    );
                    let fire_dt = class_start_dt - Duration::minutes(lead_minutes as i64);

                    if fire_dt > now {
                        let title = format!(
                            "🔔 Class in {} min: {}",
                            lead_minutes, session.course_name
                        );
                        let body = format!(
                            "{} · {} at {}\nStarts at {}",
                            session.session_type,
                            session.room_short,
                            session.venue,
                            session.start_time
                        );

                        reminders.push(Reminder {
                            id: format!("class-{}-{}", session.id, target_date),
                            session_id: Some(session.id.clone()),
                            title,
                            body,
                            fire_time_iso: fire_dt.to_rfc3339(),
                            fire_timestamp_secs: fire_dt.timestamp(),
                            is_morning_summary: false,
                        });
                    }
                }
            }
        }

        reminders.sort_by_key(|r| r.fire_timestamp_secs);
        reminders
    }
}
