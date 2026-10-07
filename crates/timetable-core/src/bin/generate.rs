use std::fs;
use std::path::Path;
use timetable_core::{
    generate_calendar_ics, generate_llms_txt, validate_timetable, TimetableData,
};

fn main() {
    println!("=== Campus Timetable Code & Data Generator ===");

    let data = TimetableData::load_default();
    println!("Loaded schedule for: {} ({})", data.program.code, data.program.group);

    // 1. Validation
    let errors = validate_timetable(&data);
    if !errors.is_empty() {
        eprintln!("Validation errors found:");
        for e in &errors {
            eprintln!("  - {}", e);
        }
    } else {
        println!("Schedule validation: PASSED (0 errors)");
    }

    // 2. Generate web/llms.txt
    let llms_content = generate_llms_txt(&data);
    let llms_path = Path::new("web/llms.txt");
    if let Ok(()) = fs::write(llms_path, &llms_content) {
        println!("Generated web/llms.txt successfully");
    }

    // 3. Generate web/schedule.ics
    let ics_content = generate_calendar_ics(&data, Some(15));
    let ics_path = Path::new("web/schedule.ics");
    if let Ok(()) = fs::write(ics_path, &ics_content) {
        println!("Generated web/schedule.ics successfully (with 15m VALARM)");
    }

    println!("All artifacts successfully generated from core crate!");
}
