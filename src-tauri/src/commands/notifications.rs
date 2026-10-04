use tauri::command;

#[command]
pub fn notify_class_start(title: String, body: String) -> Result<(), String> {
    println!("[Timetable Notification] {}: {}", title, body);
    Ok(())
}
