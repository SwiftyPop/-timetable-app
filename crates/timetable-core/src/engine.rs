use crate::model::{Session, TimetableData};
use crate::time::Clock;
use chrono::{Datelike, Timelike};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NowInfo {
    pub day_of_week_num: u32, // 0 = Sunday, 1 = Monday, ..., 6 = Saturday
    pub minute_of_day: u32,
    pub time_formatted: String,
    pub current_session: Option<Session>,
    pub next_session: Option<Session>,
    pub next_day_offset: u32,
    pub is_class_day: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FreeGap {
    pub day_index: u32,
    pub start_hour: u32,
    pub end_hour: u32,
    pub duration_hours: u32,
    pub is_friday_break: bool,
    pub next_course_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WeekStats {
    pub total_class_hours: u32,
    pub number_of_subjects: usize,
    pub total_sessions: usize,
    pub busiest_day: String,
    pub busiest_day_hours: u32,
    pub earliest_start_hour: u32,
    pub latest_end_hour: u32,
}

#[derive(Debug, Clone)]
pub struct ScheduleEngine {
    data: TimetableData,
}

impl ScheduleEngine {
    pub fn new(data: TimetableData) -> Self {
        Self { data }
    }

    pub fn data(&self) -> &TimetableData {
        &self.data
    }

    /// Retrieve sessions for a given day (1 = Monday ... 5 = Friday), sorted by start_hour.
    pub fn sessions_for_day(&self, day_index: u32) -> Vec<Session> {
        let mut list: Vec<Session> = self
            .data
            .schedule
            .iter()
            .filter(|s| s.day_index == day_index)
            .cloned()
            .collect();
        list.sort_by_key(|s| s.start_hour);
        list
    }

    /// Calculate free gaps between classes on a given day.
    pub fn free_gaps_for_day(&self, day_index: u32) -> Vec<FreeGap> {
        let sessions = self.sessions_for_day(day_index);
        let mut gaps = Vec::new();

        for i in 1..sessions.len() {
            let prev = &sessions[i - 1];
            let curr = &sessions[i];

            if curr.start_hour > prev.end_hour {
                let duration = curr.start_hour - prev.end_hour;
                let is_friday_break = day_index == 5 && prev.end_hour <= 12 && curr.start_hour >= 15;

                gaps.push(FreeGap {
                    day_index,
                    start_hour: prev.end_hour,
                    end_hour: curr.start_hour,
                    duration_hours: duration,
                    is_friday_break,
                    next_course_name: curr.course_name.clone(),
                });
            }
        }

        gaps
    }

    /// Calculates current class, next class, and status given any Clock implementation.
    pub fn now_info(&self, clock: &dyn Clock) -> NowInfo {
        let now = clock.now();
        // chrono weekday: Mon=0 .. Sun=6 -> Map to JS style: Sun=0, Mon=1 .. Sat=6
        let js_day = match now.weekday() {
            chrono::Weekday::Sun => 0,
            chrono::Weekday::Mon => 1,
            chrono::Weekday::Tue => 2,
            chrono::Weekday::Wed => 3,
            chrono::Weekday::Thu => 4,
            chrono::Weekday::Fri => 5,
            chrono::Weekday::Sat => 6,
        };

        let minute_of_day = now.hour() * 60 + now.minute();
        let time_formatted = format!("{:02}:{:02}", now.hour(), now.minute());
        let is_class_day = (1..=5).contains(&js_day);

        // Current session
        let current_session = if is_class_day {
            self.data
                .schedule
                .iter()
                .find(|s| {
                    s.day_index == js_day
                        && minute_of_day >= s.start_hour * 60
                        && minute_of_day < s.end_hour * 60
                })
                .cloned()
        } else {
            None
        };

        // Next upcoming session within 7 days
        let mut next_session = None;
        let mut next_day_offset = 0;

        for offset in 0..=7 {
            let target_day = (js_day + offset) % 7;
            if !(1..=5).contains(&target_day) {
                continue;
            }

            let mut candidates: Vec<&Session> = self
                .data
                .schedule
                .iter()
                .filter(|s| {
                    s.day_index == target_day && (offset > 0 || s.start_hour * 60 > minute_of_day)
                })
                .collect();

            if !candidates.is_empty() {
                candidates.sort_by_key(|s| s.start_hour);
                next_session = Some((*candidates[0]).clone());
                next_day_offset = offset;
                break;
            }
        }

        NowInfo {
            day_of_week_num: js_day,
            minute_of_day,
            time_formatted,
            current_session,
            next_session,
            next_day_offset,
            is_class_day,
        }
    }

    /// Week statistics: total hours, busiest day, bounds.
    pub fn week_stats(&self) -> WeekStats {
        let mut total_hours = 0;
        let mut day_hours = [0u32; 6]; // 1..=5
        let mut earliest = 24u32;
        let mut latest = 0u32;

        for s in &self.data.schedule {
            total_hours += s.duration_hours;
            if (1..=5).contains(&s.day_index) {
                day_hours[s.day_index as usize] += s.duration_hours;
            }
            if s.start_hour < earliest {
                earliest = s.start_hour;
            }
            if s.end_hour > latest {
                latest = s.end_hour;
            }
        }

        let day_names = ["", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday"];
        let mut busiest_idx = 1;
        let mut max_h = 0;
        for (i, &h) in day_hours.iter().enumerate().skip(1) {
            if h > max_h {
                max_h = h;
                busiest_idx = i;
            }
        }

        WeekStats {
            total_class_hours: total_hours,
            number_of_subjects: self.data.subjects.len(),
            total_sessions: self.data.schedule.len(),
            busiest_day: day_names[busiest_idx].to_string(),
            busiest_day_hours: max_h,
            earliest_start_hour: if earliest <= 24 { earliest } else { 8 },
            latest_end_hour: latest,
        }
    }

    /// "Crosses my time": check what is active at a given day index and minute of day.
    pub fn crosses_my_time(&self, day_index: u32, minute_of_day: u32) -> Option<&Session> {
        self.data.schedule.iter().find(|s| {
            s.day_index == day_index
                && minute_of_day >= s.start_hour * 60
                && minute_of_day < s.end_hour * 60
        })
    }
}
