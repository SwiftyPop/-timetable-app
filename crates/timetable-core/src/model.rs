use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const DEFAULT_SCHEDULE_JSON: &str = include_str!("../data/schedule.json");

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Institution {
    pub name: String,
    pub faculty: String,
    pub campus: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Program {
    pub code: String,
    pub year: u32,
    pub group: String,
    pub academic_term: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Summary {
    pub total_class_hours: u32,
    pub number_of_subjects: usize,
    pub total_sessions: usize,
    pub first_class: String,
    pub last_class: String,
    #[serde(default)]
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Subject {
    pub code: String,
    pub name: String,
    pub short_name: String,
    pub faculty: String,
    #[serde(default)]
    pub group_link: Option<String>,
    #[serde(default)]
    pub group_link_alt: Option<String>,
    #[serde(default)]
    pub coordinator: Option<String>,
    #[serde(default)]
    pub urlearn_subject: Option<String>,
    #[serde(default)]
    pub cohort: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub id: String,
    pub day_of_week: String,
    pub day_index: u32, // 1 = Monday, 2 = Tuesday, ..., 5 = Friday
    pub start_time: String,
    pub end_time: String,
    pub start_hour: u32,
    pub end_hour: u32,
    pub duration_hours: u32,
    pub course_code: String,
    pub course_name: String,
    #[serde(rename = "type")]
    pub session_type: String,
    pub venue: String,
    pub building: String,
    pub room: String,
    pub room_short: String,
    #[serde(default)]
    pub group_link: Option<String>,
    #[serde(default)]
    pub instructors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TimetableData {
    pub title: String,
    pub institution: Institution,
    pub program: Program,
    pub summary: Summary,
    pub subjects: HashMap<String, Subject>,
    #[serde(default)]
    pub community_channels: Option<HashMap<String, String>>,
    pub schedule: Vec<Session>,
}

impl TimetableData {
    pub fn from_json_str(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn load_default() -> Self {
        Self::from_json_str(DEFAULT_SCHEDULE_JSON)
            .expect("Embedded schedule.json must be valid JSON")
    }
}
