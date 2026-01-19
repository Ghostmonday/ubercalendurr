use std::collections::VecDeque;
use std::sync::Arc;

pub mod parser;

use parser::{SimpleParser, ParsedEvent};

pub enum ParserStrategy {
    SimpleParser,
    AIParser,
}

pub struct CommandParser;

impl CommandParser {
    pub fn is_json(&self, input: &str) -> bool {
        let trimmed = input.trim();
        trimmed.starts_with('{') && trimmed.ends_with('}')
    }

    pub fn is_command(&self, input: &str) -> bool {
        input.starts_with('/')
    }

    pub fn parse_command(&self, input: &str) -> Option<Command> {
        let parts: Vec<&str> = input.splitn(2, ' ').collect();
        match parts[0] {
            "/help" => Some(Command::Help),
            "/today" => Some(Command::ShowToday),
            "/search" => Some(Command::Search(parts.get(1).map(|s| s.to_string()).unwrap_or_default())),
            "/settings" => Some(Command::Settings),
            "/clear" => Some(Command::Clear),
            "/export" => Some(Command::Export(parts.get(1).map(|s| s.to_string()).unwrap_or_default())),
            "/exit" | "/quit" => Some(Command::Exit),
            _ => None,
        }
    }
}

pub enum Command {
    Help,
    ShowToday,
    Search(String),
    Settings,
    Clear,
    Export(String),
    Exit,
}

pub struct InputHandler {
    simple_parser: SimpleParser,
    command_parser: CommandParser,
    input_history: VecDeque<String>,
    history_position: Option<usize>,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            simple_parser: SimpleParser,
            command_parser: CommandParser,
            input_history: VecDeque::with_capacity(100),
            history_position: None,
        }
    }

    pub fn parse(&self, input: &str) -> Result<ParsedEvent, String> {
        let trimmed = input.trim();

        if trimmed.is_empty() {
            return Err("Empty input".to_string());
        }

        // Try SimpleParser first (always works offline)
        self.simple_parser.parse(trimmed)
    }

    pub fn handle_input(&mut self, input: &str) -> InputResult {
        let trimmed = input.trim();

        if trimmed.is_empty() {
            return InputResult::Clear;
        }

        if let Some(command) = self.command_parser.parse_command(trimmed) {
            return self.execute_command(command);
        }

        if self.command_parser.is_json(trimmed) {
            return InputResult::Info("JSON input detected".to_string());
        }

        InputResult::Processing(trimmed.to_string())
    }

    fn execute_command(&self, command: Command) -> InputResult {
        match command {
            Command::Help => InputResult::ShowHelp,
            Command::ShowToday => InputResult::Info("Showing today's events".to_string()),
            Command::Search(query) => InputResult::Search(query),
            Command::Settings => InputResult::OpenSettings,
            Command::Clear => InputResult::Clear,
            Command::Export(format) => InputResult::Export(format),
            Command::Exit => InputResult::Exit,
        }
    }

    pub fn history_up(&mut self) -> Option<&str> {
        if let Some(pos) = self.history_position {
            if pos > 0 {
                self.history_position = Some(pos - 1);
                return self.input_history.get(pos - 1).map(|s| s.as_str());
            }
        }
        None
    }

    pub fn history_down(&mut self) -> Option<&str> {
        if let Some(pos) = self.history_position {
            if pos < self.input_history.len() - 1 {
                self.history_position = Some(pos + 1);
                return self.input_history.get(pos + 1).map(|s| s.as_str());
            } else {
                self.history_position = None;
            }
        }
        None
    }
}

pub enum InputResult {
    Processing(String),
    ShowHelp,
    Info(String),
    Search(String),
    OpenSettings,
    Clear,
    Export(String),
    Exit,
    Error(String),
}
