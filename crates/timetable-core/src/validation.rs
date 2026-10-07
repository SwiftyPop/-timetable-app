use crate::model::TimetableData;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Error, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind", content = "message")]
pub enum ValidationError {
    #[error("Session '{session_id}' has invalid time range: {start_hour}:00 to {end_hour}:00")]
    InvalidTimeRange {
        session_id: String,
        start_hour: u32,
        end_hour: u32,
    },
    #[error("Session '{session_id}' is scheduled out of hours ({start_hour}:00 to {end_hour}:00). Expected 08:00 - 22:00")]
    OutOfHours {
        session_id: String,
        start_hour: u32,
        end_hour: u32,
    },
    #[error("Session '{session_id}' is scheduled on weekend (day index {day_index})")]
    WeekendSession {
        session_id: String,
        day_index: u32,
    },
    #[error("Sessions '{session_a_id}' and '{session_b_id}' overlap on day {day_index} ({start_a}:00-{end_a}:00 vs {start_b}:00-{end_b}:00)")]
    OverlappingSessions {
        day_index: u32,
        session_a_id: String,
        session_b_id: String,
        start_a: u32,
        end_a: u32,
        start_b: u32,
        end_b: u32,
    },
}

pub fn validate_timetable(data: &TimetableData) -> Vec<ValidationError> {
    let mut errors = Vec::new();

    for session in &data.schedule {
        // 1. Valid range
        if session.start_hour >= session.end_hour {
            errors.push(ValidationError::InvalidTimeRange {
                session_id: session.id.clone(),
                start_hour: session.start_hour,
                end_hour: session.end_hour,
            });
        }

        // 2. Out of normal operating hours (08:00 to 22:00)
        if session.start_hour < 8 || session.end_hour > 22 {
            errors.push(ValidationError::OutOfHours {
                session_id: session.id.clone(),
                start_hour: session.start_hour,
                end_hour: session.end_hour,
            });
        }

        // 3. Weekend session
        if session.day_index < 1 || session.day_index > 5 {
            errors.push(ValidationError::WeekendSession {
                session_id: session.id.clone(),
                day_index: session.day_index,
            });
        }
    }

    // 4. Overlap checks
    for i in 0..data.schedule.len() {
        for j in (i + 1)..data.schedule.len() {
            let a = &data.schedule[i];
            let b = &data.schedule[j];

            if a.day_index == b.day_index {
                let max_start = a.start_hour.max(b.start_hour);
                let min_end = a.end_hour.min(b.end_hour);

                if max_start < min_end {
                    errors.push(ValidationError::OverlappingSessions {
                        day_index: a.day_index,
                        session_a_id: a.id.clone(),
                        session_b_id: b.id.clone(),
                        start_a: a.start_hour,
                        end_a: a.end_hour,
                        start_b: b.start_hour,
                        end_b: b.end_hour,
                    });
                }
            }
        }
    }

    errors
}
