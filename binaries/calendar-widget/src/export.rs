use calendar_core::CalendarEvent;
use std::io::Write;
use std::fs::File;
use std::path::PathBuf;

pub struct Exporter;

impl Exporter {
    pub fn export_json(events: &[CalendarEvent], path: &PathBuf) -> Result<(), String> {
        let json = serde_json::to_string_pretty(events)
            .map_err(|e| format!("JSON serialization failed: {}", e))?;
        
        let mut file = File::create(path)
            .map_err(|e| format!("Failed to create file: {}", e))?;
        
        file.write_all(json.as_bytes())
            .map_err(|e| format!("Failed to write file: {}", e))?;
        
        Ok(())
    }

    pub fn export_csv(events: &[CalendarEvent], path: &PathBuf) -> Result<(), String> {
        let mut file = File::create(path)
            .map_err(|e| format!("Failed to create file: {}", e))?;
        
        // Write CSV header
        writeln!(file, "Date,Time,Event,Category,Priority,Notes")
            .map_err(|e| format!("Failed to write CSV header: {}", e))?;
        
        // Write events
        for event in events {
            let time_str = event.time.as_deref().unwrap_or("");
            let notes_str = event.notes.as_deref().unwrap_or("").replace(',', ";");
            
            writeln!(
                file,
                "{},{},\"{}\",{},{},\"{}\"",
                event.date,
                time_str,
                event.event.replace('"', "\"\""),
                event.category.as_str(),
                event.priority.as_str(),
                notes_str
            )
            .map_err(|e| format!("Failed to write CSV row: {}", e))?;
        }
        
        Ok(())
    }

    pub fn export_ics(events: &[CalendarEvent], path: &PathBuf) -> Result<(), String> {
        let mut file = File::create(path)
            .map_err(|e| format!("Failed to create file: {}", e))?;
        
        // Write iCalendar header
        writeln!(file, "BEGIN:VCALENDAR")
            .map_err(|e| format!("Failed to write ICS header: {}", e))?;
        writeln!(file, "VERSION:2.0")
            .map_err(|e| format!("Failed to write ICS version: {}", e))?;
        writeln!(file, "PRODID:-//UberCalendurr//EN")
            .map_err(|e| format!("Failed to write ICS prodid: {}", e))?;
        
        // Write events
        for event in events {
            writeln!(file, "BEGIN:VEVENT")
                .map_err(|e| format!("Failed to write VEVENT start: {}", e))?;
            
            writeln!(file, "UID:{}", event.id)
                .map_err(|e| format!("Failed to write UID: {}", e))?;
            
            writeln!(file, "DTSTART:{}", Self::format_ics_datetime(&event.date, event.time.as_deref()))
                .map_err(|e| format!("Failed to write DTSTART: {}", e))?;
            
            if let Some(end_time) = &event.end_time {
                writeln!(file, "DTEND:{}", Self::format_ics_datetime(&event.date, Some(end_time)))
                    .map_err(|e| format!("Failed to write DTEND: {}", e))?;
            }
            
            writeln!(file, "SUMMARY:{}", Self::escape_ics_text(&event.event))
                .map_err(|e| format!("Failed to write SUMMARY: {}", e))?;
            
            if let Some(notes) = &event.notes {
                writeln!(file, "DESCRIPTION:{}", Self::escape_ics_text(notes))
                    .map_err(|e| format!("Failed to write DESCRIPTION: {}", e))?;
            }
            
            writeln!(file, "STATUS:{}", match event.status {
                calendar_core::EventStatus::Confirmed => "CONFIRMED",
                calendar_core::EventStatus::Tentative => "TENTATIVE",
                calendar_core::EventStatus::Cancelled => "CANCELLED",
                calendar_core::EventStatus::Completed => "COMPLETED",
            })
            .map_err(|e| format!("Failed to write STATUS: {}", e))?;
            
            writeln!(file, "END:VEVENT")
                .map_err(|e| format!("Failed to write VEVENT end: {}", e))?;
        }
        
        // Write iCalendar footer
        writeln!(file, "END:VCALENDAR")
            .map_err(|e| format!("Failed to write ICS footer: {}", e))?;
        
        Ok(())
    }

    fn format_ics_datetime(date: &str, time: Option<&str>) -> String {
        if let Some(t) = time {
            format!("{}T{}00", date.replace('-', ""), t.replace(':', ""))
        } else {
            format!("{}T000000", date.replace('-', ""))
        }
    }

    fn escape_ics_text(text: &str) -> String {
        text.replace('\\', "\\\\")
            .replace(',', "\\,")
            .replace(';', "\\;")
            .replace('\n', "\\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use calendar_core::{CalendarEvent, Priority, Category};
    
    fn create_test_events() -> Vec<CalendarEvent> {
        vec![
            {
                let mut e = CalendarEvent::new("Event 1".to_string(), "2026-01-20".to_string());
                e.time = Some("14:00".to_string());
                e.priority = Priority::High;
                e.category = Category::Work;
                e
            },
            {
                let mut e = CalendarEvent::new("Event 2".to_string(), "2026-01-21".to_string());
                e.time = Some("12:00".to_string());
                e.notes = Some("Lunch notes".to_string());
                e
            },
        ]
    }
    
    #[test]
    fn test_export_json() {
        let events = create_test_events();
        let path = PathBuf::from("test_export.json");
        
        Exporter::export_json(&events, &path).unwrap();
        
        let content = fs::read_to_string(&path).unwrap();
        assert!(content.contains("Event 1"));
        assert!(content.contains("2026-01-20"));
        
        fs::remove_file(&path).unwrap();
    }
    
    #[test]
    fn test_export_csv() {
        let events = create_test_events();
        let path = PathBuf::from("test_export.csv");
        
        Exporter::export_csv(&events, &path).unwrap();
        
        let content = fs::read_to_string(&path).unwrap();
        assert!(content.contains("Date,Time,Event,Category,Priority,Notes"));
        assert!(content.contains("Event 1"));
        assert!(content.contains("14:00"));
        
        fs::remove_file(&path).unwrap();
    }
    
    #[test]
    fn test_export_ics() {
        let events = create_test_events();
        let path = PathBuf::from("test_export.ics");
        
        Exporter::export_ics(&events, &path).unwrap();
        
        let content = fs::read_to_string(&path).unwrap();
        assert!(content.contains("BEGIN:VCALENDAR"));
        assert!(content.contains("BEGIN:VEVENT"));
        assert!(content.contains("SUMMARY:Event 1"));
        assert!(content.contains("END:VCALENDAR"));
        
        fs::remove_file(&path).unwrap();
    }
    
    #[test]
    fn test_ics_datetime_formatting() {
        assert_eq!(
            Exporter::format_ics_datetime("2026-01-20", Some("14:00")),
            "20260120T140000"
        );
        assert_eq!(
            Exporter::format_ics_datetime("2026-01-20", None),
            "20260120T000000"
        );
    }
    
    #[test]
    fn test_ics_text_escaping() {
        assert_eq!(Exporter::escape_ics_text("Hello, world"), "Hello\\, world");
        assert_eq!(Exporter::escape_ics_text("Line1\nLine2"), "Line1\\nLine2");
        assert_eq!(Exporter::escape_ics_text("Path\\to\\file"), "Path\\\\to\\\\file");
    }
}
