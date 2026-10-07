use crate::model::TimetableData;

/// Generate JS representations for `S` (subjects) and `E` (events) from the timetable data.
pub fn generate_js_data(data: &TimetableData) -> String {
    // Generate S object
    // Map of subject key -> { n: name, c: code, h: hue }
    // Hues used: ctrl: 34, ect: 268, el2: 196, mi: 152, mt: 345
    let mut s_entries = Vec::new();
    let hues = [
        ("EMK22003", "ctrl", 34),
        ("EMK21203", "ect", 268),
        ("EMK21303", "el2", 196),
        ("EMK21103", "mi", 152),
        ("IMQ21303", "mt", 345),
    ];

    for (code_prefix, key, hue) in &hues {
        if let Some((_, subj)) = data.subjects.iter().find(|(k, _)| k.contains(code_prefix)) {
            s_entries.push(format!(
                "{}:{{n:'{}',c:'{}',h:{}}}",
                key, subj.name, subj.code, hue
            ));
        }
    }

    // Generate E array
    // [dayIndex, startHour, endHour, subjectKey, sessionType, venue, lecturers]
    let mut e_entries = Vec::new();
    for session in &data.schedule {
        let key = if session.course_code.contains("EMK22003") {
            "ctrl"
        } else if session.course_code.contains("EMK21203") {
            "ect"
        } else if session.course_code.contains("EMK21303") {
            "el2"
        } else if session.course_code.contains("EMK21103") {
            "mi"
        } else {
            "mt"
        };

        let instructors = session.instructors.join(", ");
        e_entries.push(format!(
            "  [{},{},{},'{}','{}','{}','{}']",
            session.day_index,
            session.start_hour,
            session.end_hour,
            key,
            session.session_type,
            session.venue,
            instructors
        ));
    }

    format!(
        "var S={{{}}};\nvar E=[\n{}\n];",
        s_entries.join(","),
        e_entries.join(",\n")
    )
}

/// Generate llms.txt content from TimetableData single source of truth.
pub fn generate_llms_txt(data: &TimetableData) -> String {
    let mut out = String::new();
    out.push_str(&format!("# {}\n\n", data.title));
    out.push_str(&format!("> **Institution:** {}\n", data.institution.name));
    out.push_str(&format!("> **Faculty:** {}\n", data.institution.faculty));
    out.push_str(&format!("> **Campus:** {}\n", data.institution.campus));
    out.push_str(&format!("> **Program:** {} ({})\n", data.program.code, data.program.group));
    out.push_str(&format!("> **Term:** {}\n\n", data.program.academic_term));

    out.push_str("## Weekly Class Schedule\n\n");
    out.push_str("| Day | Time | Course | Type | Venue | Instructors |\n");
    out.push_str("| --- | --- | --- | --- | --- | --- |\n");

    let mut sorted = data.schedule.clone();
    sorted.sort_by_key(|s| (s.day_index, s.start_hour));

    for s in &sorted {
        let instructors = if s.instructors.is_empty() {
            "-".to_string()
        } else {
            s.instructors.join(", ")
        };
        out.push_str(&format!(
            "| {} | {}-{} | {} ({}) | {} | {} | {} |\n",
            s.day_of_week, s.start_time, s.end_time, s.course_name, s.course_code, s.session_type, s.venue, instructors
        ));
    }

    out.push_str("\n## Summary\n");
    out.push_str(&format!("- Total class hours: {}\n", data.summary.total_class_hours));
    out.push_str(&format!("- Total subjects: {}\n", data.summary.number_of_subjects));
    out.push_str(&format!("- Total sessions: {}\n", data.summary.total_sessions));
    if let Some(note) = &data.summary.note {
        out.push_str(&format!("- Note: {}\n", note));
    }

    out
}
