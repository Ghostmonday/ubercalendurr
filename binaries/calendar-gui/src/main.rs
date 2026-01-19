#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;
use std::sync::Arc;
use tauri::{Manager, State};
use storage_engine::CalendarRepository;
use calendar_core::CalendarEvent;

struct AppState {
    repository: Arc<CalendarRepository>,
}

#[tauri::command]
async fn get_events(
    start_date: String,
    end_date: String,
    state: State<'_, AppState>,
) -> Result<Vec<CalendarEvent>, String> {
    let repository = state.repository.clone();
    
    tokio::task::spawn_blocking(move || {
        repository.get_by_date_range(&start_date, &end_date)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
    .map_err(|e| format!("Failed to get events: {}", e))
}

#[tauri::command]
async fn create_event(
    event_data: serde_json::Value,
    state: State<'_, AppState>,
) -> Result<CalendarEvent, String> {
    // Parse event from JSON
    let event: CalendarEvent = serde_json::from_value(event_data)
        .map_err(|e| format!("Invalid event data: {}", e))?;
    
    // Validate
    event.validate()
        .map_err(|e| format!("Event validation failed: {}", e))?;
    
    // Save to database (spawn_blocking for sync repository)
    let repository = state.repository.clone();
    let event_clone = event.clone();
    
    tokio::task::spawn_blocking(move || {
        repository.save_event(&event_clone)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
    .map_err(|e| format!("Failed to save event: {}", e))?;
    
    Ok(event)
}

#[tauri::command]
async fn update_event(
    event_id: String,
    event_data: serde_json::Value,
    state: State<'_, AppState>,
) -> Result<CalendarEvent, String> {
    let mut event: CalendarEvent = serde_json::from_value(event_data)
        .map_err(|e| format!("Invalid event data: {}", e))?;
    
    // Ensure ID matches
    event.id = event_id.parse()
        .map_err(|_| "Invalid event ID".to_string())?;
    event.updated_at = chrono::Utc::now();
    
    event.validate()
        .map_err(|e| format!("Event validation failed: {}", e))?;
    
    let repository = state.repository.clone();
    let event_clone = event.clone();
    
    tokio::task::spawn_blocking(move || {
        repository.save_event(&event_clone)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
    .map_err(|e| format!("Failed to update event: {}", e))?;
    
    Ok(event)
}

#[tauri::command]
async fn delete_event(
    event_id: String,
    state: State<'_, AppState>,
) -> Result<bool, String> {
    let repository = state.repository.clone();
    
    tokio::task::spawn_blocking(move || {
        repository.delete_event(&event_id)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
    .map_err(|e| format!("Failed to delete event: {}", e))
}

#[tauri::command]
async fn search_events(
    query: String,
    state: State<'_, AppState>,
) -> Result<Vec<CalendarEvent>, String> {
    // Simple search - get all events and filter (can be optimized later)
    let today = chrono::Local::now().date_naive();
    let start_date = (today - chrono::Duration::days(365)).format("%Y-%m-%d").to_string();
    let end_date = (today + chrono::Duration::days(365)).format("%Y-%m-%d").to_string();
    
    let repository = state.repository.clone();
    
    let events = tokio::task::spawn_blocking(move || {
        repository.get_by_date_range(&start_date, &end_date)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
    .map_err(|e| format!("Failed to search events: {}", e))?;
    
    let query_lower = query.to_lowercase();
    let filtered: Vec<CalendarEvent> = events
        .into_iter()
        .filter(|e| {
            e.event.to_lowercase().contains(&query_lower) ||
            e.notes.as_ref().map(|n| n.to_lowercase().contains(&query_lower)).unwrap_or(false) ||
            e.tags.iter().any(|t| t.to_lowercase().contains(&query_lower))
        })
        .collect();
    
    Ok(filtered)
}

fn main() {
    // Determine database path (same as widget)
    let db_path = directories::BaseDirs::new()
        .map(|d| d.config_dir().join("ubercalendurr").join("calendar.db"))
        .unwrap_or_else(|| PathBuf::from("calendar.db"));
    
    // Initialize repository (synchronous now)
    let repository = CalendarRepository::new(&db_path)
        .expect("Failed to initialize database");
    
    let app_state = AppState {
        repository: Arc::new(repository),
    };
    
    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            get_events,
            create_event,
            update_event,
            delete_event,
            search_events
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
