use std::sync::Arc;
use crate::AppState;
use crate::input::{InputHandler, Command};
use crate::input::parser::ParsedEvent;
use crate::export::Exporter;
use calendar_core::CalendarEvent;
use uuid::Uuid;
use std::path::PathBuf;

pub struct App {
    state: Arc<AppState>,
    input_handler: InputHandler,
}

impl App {
    pub fn new(state: Arc<AppState>) -> Result<Self, std::io::Error> {
        Ok(Self { 
            state,
            input_handler: InputHandler::new(),
        })
    }

    /// Start background task to check for upcoming notifications
    pub fn start_notification_checker(&self) {
        let state = self.state.clone();
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(60));
            
            loop {
                interval.tick().await;
                
                // Get today's and tomorrow's events
                let today = chrono::Local::now().date_naive();
                let tomorrow = today + chrono::Duration::days(1);
                let start_date = today.format("%Y-%m-%d").to_string();
                let end_date = tomorrow.format("%Y-%m-%d").to_string();
                
                let repository = state.repository.clone();
                
                match tokio::task::spawn_blocking(move || {
                    repository.0.get_by_date_range(&start_date, &end_date)
                }).await {
                    Ok(Ok(events)) => {
                        for event in events {
                            if state.notification_service.should_notify(
                                &event,
                                state.settings.notifications.default_reminder_minutes
                            ) {
                                if let Err(e) = state.notification_service.send_notification(&event) {
                                    eprintln!("Failed to send notification: {}", e);
                                }
                            }
                        }
                    }
                    _ => {} // Ignore errors in background task
                }
            }
        });
    }
    
    pub async fn run(&mut self) -> Result<(), std::io::Error> {
        println!("UberCalendurr Widget v0.1.0");
        println!("Type /help for available commands");
        println!();
        
        // Start background notification checker
        self.start_notification_checker();
        
        self.run_interactive().await
    }

    async fn run_interactive(&mut self) -> Result<(), std::io::Error> {
        use std::io::{self, Write};

        let stdin = io::stdin();
        let mut input = String::new();

        loop {
            print!("📅> ");
            io::stdout().flush()?;

            input.clear();
            let bytes_read = stdin.read_line(&mut input)?;

            if bytes_read == 0 {
                break;
            }

            let input_str = input.trim().to_string();

            if input_str.is_empty() {
                continue;
            }

            // Handle commands
            if let Some(command) = self.input_handler.command_parser.parse_command(&input_str) {
                match command {
                    Command::Help => {
                        println!("Available commands:");
                        println!("  /help          - Show this help");
                        println!("  /today         - Show today's events");
                        println!("  /search <term> - Search events");
                        println!("  /export <fmt>  - Export events (json/csv/ics)");
                        println!("  /exit          - Exit application");
                        continue;
                    }
                    Command::ShowToday => {
                        self.show_today_events().await?;
                        continue;
                    }
                    Command::Search(query) => {
                        println!("Search: {}", query);
                        // TODO: Implement search
                        continue;
                    }
                    Command::Export(format) => {
                        self.handle_export(&format).await?;
                        continue;
                    }
                    Command::Settings => {
                        println!("Settings (not implemented yet)");
                        continue;
                    }
                    Command::Clear => {
                        continue;
                    }
                    Command::Exit => {
                        break;
                    }
                }
            }

            // Handle JSON input
            if self.input_handler.command_parser.is_json(&input_str) {
                println!("JSON input detected - parsing...");
                // TODO: Parse JSON directly
                continue;
            }

            // Handle natural language input
            match self.input_handler.parse(&input_str) {
                Ok(parsed_event) => {
                    // Convert to CalendarEvent
                    let event = CalendarEvent::from_parsed(
                        parsed_event.event,
                        parsed_event.date,
                        parsed_event.time,
                        parsed_event.end_time,
                        parsed_event.notes,
                        parsed_event.priority,
                        parsed_event.category,
                        parsed_event.tags,
                        parsed_event.metadata,
                        parsed_event.recurring,
                    );

                    // Validate
                    if let Err(e) = event.validate() {
                        println!("❌ Validation error: {}", e);
                        continue;
                    }

                    // Save to database (spawn_blocking for sync repository)
                    let repository = self.state.repository.clone();
                    let event_clone = event.clone();
                    
                    match tokio::task::spawn_blocking(move || {
                        repository.save_event(&event_clone)
                    }).await {
                        Ok(Ok(_)) => {
                            // Success confirmation
                            let time_str = event.time.as_deref().unwrap_or("--:--");
                            let category_str = event.category.as_str();
                            println!(
                                "✅ [{} {}] {} ({}) — Saved.",
                                event.date,
                                time_str,
                                event.event,
                                category_str
                            );
                        }
                        Ok(Err(e)) => {
                            println!("❌ Failed to save: {}", e);
                        }
                        Err(e) => {
                            println!("❌ Task error: {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("❌ Failed to parse: {}", e);
                    println!("   Try: 'Meeting tomorrow at 2pm' or 'Lunch with Sarah next Tuesday'");
                }
            }
        }

        Ok(())
    }

    async fn show_today_events(&self) -> Result<(), std::io::Error> {
        let repository = self.state.repository.clone();
        
        match tokio::task::spawn_blocking(move || {
            repository.get_today_events()
        }).await {
            Ok(Ok(events)) => {
                if events.is_empty() {
                    println!("No events scheduled for today.");
                } else {
                    println!("Today's events:");
                    for event in events {
                        let time_str = event.time.as_deref().unwrap_or("--:--");
                        println!("  [{}] {} ({})", time_str, event.event, event.category.as_str());
                    }
                }
            }
            Ok(Err(e)) => {
                println!("❌ Failed to load events: {}", e);
            }
            Err(e) => {
                println!("❌ Task error: {}", e);
            }
        }
        
        Ok(())
    }

    async fn handle_export(&self, format: &str) -> Result<(), std::io::Error> {
        // Get all events
        let repository = self.state.repository.clone();
        let today = chrono::Local::now().date_naive();
        let start_date = (today - chrono::Duration::days(365)).format("%Y-%m-%d").to_string();
        let end_date = (today + chrono::Duration::days(365)).format("%Y-%m-%d").to_string();
        
        let events = match tokio::task::spawn_blocking(move || {
            repository.0.get_by_date_range(&start_date, &end_date)
        }).await {
            Ok(Ok(events)) => events,
            Ok(Err(e)) => {
                println!("❌ Failed to load events: {}", e);
                return Ok(());
            }
            Err(e) => {
                println!("❌ Task error: {}", e);
                return Ok(());
            }
        };

        // Generate filename
        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
        let extension = match format.to_lowercase().as_str() {
            "json" => "json",
            "csv" => "csv",
            "ics" => "ics",
            _ => {
                println!("❌ Unsupported format: {}. Use json, csv, or ics", format);
                return Ok(());
            }
        };
        
        let filename = format!("ubercalendurr_export_{}.{}", timestamp, extension);
        let export_path = PathBuf::from(&filename);

        // Export
        let result = match format.to_lowercase().as_str() {
            "json" => Exporter::export_json(&events, &export_path),
            "csv" => Exporter::export_csv(&events, &export_path),
            "ics" => Exporter::export_ics(&events, &export_path),
            _ => unreachable!(),
        };

        match result {
            Ok(_) => {
                println!("✅ Exported {} events to {}", events.len(), filename);
            }
            Err(e) => {
                println!("❌ Export failed: {}", e);
            }
        }

        Ok(())
    }
}
