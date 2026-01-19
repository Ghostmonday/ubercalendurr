use regex::Regex;
use once_cell::sync::Lazy;

static TIME_PATTERN: Lazy<Regex> = 
    Lazy::new(|| Regex::new(r"^(?:[01]?[0-9]|2[0-3]):[0-5][0-9]$").unwrap());

static DATE_PATTERN: Lazy<Regex> = 
    Lazy::new(|| Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap());

pub struct Validator;

impl Validator {
    pub fn validate_time(time: &str) -> Result<(), String> {
        if !TIME_PATTERN.is_match(time) {
            Err(format!("Invalid time format: {}", time))
        } else {
            Ok(())
        }
    }

    pub fn validate_date(date: &str) -> Result<(), String> {
        if !DATE_PATTERN.is_match(date) {
            Err(format!("Invalid date format: {}", date))
        } else {
            Ok(())
        }
    }

    pub fn validate_hex_color(color: &str) -> Result<(), String> {
        if !color.starts_with('#') || color.len() != 7 {
            Err(format!("Invalid hex color: {}", color))
        } else {
            Ok(())
        }
    }

    pub fn sanitize_input(input: &str) -> String {
        input.trim().to_string()
    }
}
