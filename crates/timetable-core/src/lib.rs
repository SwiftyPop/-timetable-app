pub mod engine;
pub mod generators;
pub mod ics;
pub mod model;
pub mod reminders;
pub mod time;
pub mod validation;

pub use engine::{FreeGap, NowInfo, ScheduleEngine, WeekStats};
pub use generators::{generate_js_data, generate_llms_txt};
pub use ics::{fold_line, generate_calendar_ics, generate_vevent};
pub use model::{Session, Subject, TimetableData};
pub use reminders::{Reminder, ReminderPlanner};
pub use time::{malaysia_offset, Clock, FakeClock, SystemClock};
pub use validation::{validate_timetable, ValidationError};

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_engine() -> ScheduleEngine {
        let data = TimetableData::load_default();
        ScheduleEngine::new(data)
    }

    #[test]
    fn test_load_default_schedule_validity() {
        let data = TimetableData::load_default();
        assert_eq!(data.program.code, "UR6522002");
        assert_eq!(data.schedule.len(), 10);
        let errors = validate_timetable(&data);
        assert!(errors.is_empty(), "Default timetable should have 0 validation errors: {:?}", errors);
    }

    #[test]
    fn test_fake_clock_weekend() {
        let engine = get_test_engine();
        // Saturday 2026-10-10 at 14:00 (no classes on Saturday)
        let clock = FakeClock::from_ymd_hms(2026, 10, 10, 14, 0, 0);
        let info = engine.now_info(&clock);

        assert_eq!(info.day_of_week_num, 6); // Saturday
        assert!(!info.is_class_day);
        assert!(info.current_session.is_none(), "Should have no class currently on Saturday");
        assert!(info.next_session.is_some(), "Should find next session for upcoming week");
        let next = info.next_session.unwrap();
        // First class of the week is Tuesday 11:00 (Control System Technology Lab)
        assert_eq!(next.day_index, 2);
        assert_eq!(next.start_hour, 11);
        assert_eq!(info.next_day_offset, 3); // Sat -> Tue is +3 days
    }

    #[test]
    fn test_fake_clock_in_class() {
        let engine = get_test_engine();
        // Tuesday 2026-10-06 at 12:30 (during Tuesday 11:00-14:00 Lab)
        let clock = FakeClock::from_ymd_hms(2026, 10, 6, 12, 30, 0);
        let info = engine.now_info(&clock);

        assert_eq!(info.day_of_week_num, 2);
        assert!(info.is_class_day);
        assert!(info.current_session.is_some());
        let cur = info.current_session.unwrap();
        assert_eq!(cur.course_name, "Control System Technology");
        assert_eq!(cur.start_hour, 11);
        assert_eq!(cur.end_hour, 14);

        // Next class should be Tuesday 15:00-17:00 (Maths)
        assert!(info.next_session.is_some());
        let next = info.next_session.unwrap();
        assert_eq!(next.course_name, "Mathematics for Eng. Technology 3");
        assert_eq!(next.start_hour, 15);
        assert_eq!(info.next_day_offset, 0);
    }

    #[test]
    fn test_midnight_rollover() {
        let engine = get_test_engine();
        // Monday 2026-10-05 at 23:59 (no classes on Monday, next class is Tuesday)
        let clock = FakeClock::from_ymd_hms(2026, 10, 5, 23, 59, 0);
        let info1 = engine.now_info(&clock);
        assert_eq!(info1.day_of_week_num, 1);
        assert_eq!(info1.next_day_offset, 1);

        // Advance 2 minutes to Tuesday 00:01
        clock.advance_minutes(2);
        let info2 = engine.now_info(&clock);
        assert_eq!(info2.day_of_week_num, 2);
        assert_eq!(info2.next_day_offset, 0);
        assert_eq!(info2.next_session.unwrap().start_hour, 11);
    }

    #[test]
    fn test_free_gaps_calculation() {
        let engine = get_test_engine();
        // Thursday has classes:
        // 08:00 - 10:00 (Lecture C)
        // 11:00 - 14:00 (Lab) -> 1 hour gap from 10:00 to 11:00
        // 14:00 - 17:00 (Lab) -> Back-to-back with previous lab (0 gap)
        // 17:00 - 19:00 (Lecture A) -> Back-to-back (0 gap)
        let gaps = engine.free_gaps_for_day(4);
        assert_eq!(gaps.len(), 1, "Thursday should have exactly 1 free gap");
        assert_eq!(gaps[0].start_hour, 10);
        assert_eq!(gaps[0].end_hour, 11);
        assert_eq!(gaps[0].duration_hours, 1);
    }

    #[test]
    fn test_friday_break_detection() {
        let engine = get_test_engine();
        // Friday has classes:
        // 08:00 - 10:00
        // 15:00 - 17:00
        // There is a 5-hour gap from 10:00 to 15:00, which covers the Friday break (12:00-15:00)
        let gaps = engine.free_gaps_for_day(5);
        assert_eq!(gaps.len(), 1);
        assert_eq!(gaps[0].start_hour, 10);
        assert_eq!(gaps[0].end_hour, 15);
        assert!(gaps[0].is_friday_break, "Friday gap 10:00-15:00 must be recognized as Friday Break");
    }

    #[test]
    fn test_week_stats() {
        let engine = get_test_engine();
        let stats = engine.week_stats();
        assert_eq!(stats.total_class_hours, 23);
        assert_eq!(stats.number_of_subjects, 5);
        assert_eq!(stats.total_sessions, 10);
        assert_eq!(stats.earliest_start_hour, 8);
        assert_eq!(stats.latest_end_hour, 19);
    }

    #[test]
    fn test_reminders_planning() {
        let data = TimetableData::load_default();
        let planner = ReminderPlanner::new(&data);
        // Start on Monday 2026-10-05 at 06:00
        let clock = FakeClock::from_ymd_hms(2026, 10, 5, 6, 0, 0);
        let reminders = planner.compute_reminders(clock.now(), 15, true, &[], 7);

        assert!(!reminders.is_empty());
        // Check that reminders contain morning summary and class reminders
        let summaries: Vec<_> = reminders.iter().filter(|r| r.is_morning_summary).collect();
        assert!(!summaries.is_empty(), "Should generate morning summaries for class days");

        let class_reminders: Vec<_> = reminders.iter().filter(|r| !r.is_morning_summary).collect();
        assert!(!class_reminders.is_empty(), "Should generate 15-minute lead class reminders");
    }

    #[test]
    fn test_ics_line_folding_and_valarm() {
        let long_line = "DESCRIPTION:This is a very long line of text that definitely exceeds seventy-five octets in length and should be properly folded using CRLF and a space.";
        let folded = fold_line(long_line);
        assert!(folded.contains("\r\n "));
        for segment in folded.split("\r\n") {
            assert!(segment.len() <= 75, "Each segment must be <= 75 bytes: '{}'", segment);
        }

        let data = TimetableData::load_default();
        let ics = generate_calendar_ics(&data, Some(15));
        assert!(ics.contains("BEGIN:VCALENDAR"));
        assert!(ics.contains("BEGIN:VALARM"));
        assert!(ics.contains("TRIGGER:-PT15M"));
        assert!(ics.contains("UNTIL=20270228T160000Z"));
        assert!(ics.contains("END:VCALENDAR"));
    }

    #[test]
    fn test_validation_detects_overlap() {
        let mut data = TimetableData::load_default();
        // Inject an artificial overlapping session on Tuesday (12:00 to 13:00)
        let mut overlapping = data.schedule[0].clone();
        overlapping.id = "fake-overlap".to_string();
        overlapping.start_hour = 12;
        overlapping.end_hour = 13;
        data.schedule.push(overlapping);

        let errors = validate_timetable(&data);
        assert!(!errors.is_empty());
        let has_overlap = errors.iter().any(|e| matches!(e, ValidationError::OverlappingSessions { .. }));
        assert!(has_overlap, "Validation should catch overlapping sessions");
    }
}
