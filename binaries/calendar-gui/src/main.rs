#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to UberCalendurr!", name)
}

#[tauri::command]
fn get_events(start_date: String, end_date: String) -> Result<Vec<calendar_core::CalendarEvent>, String> {
    // TODO: Implement actual event retrieval
    Ok(vec![])
}

#[tauri::command]
fn create_event(event: serde_json::Value) -> Result<calendar_core::CalendarEvent, String> {
    // TODO: Implement event creation
    Ok(calendar_core::CalendarEvent::new(
        event.get("event").and_then(|v| v.as_str()).unwrap_or("New Event").to_string(),
        event.get("date").and_then(|v| v.as_str()).unwrap_or("2024-01-01").to_string(),
    ))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, get_events, create_event])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
