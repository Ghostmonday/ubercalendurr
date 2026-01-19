use std::collections::VecDeque;
use crate::input::parser::InputParser;

pub mod parser {
    use regex::Regex;
    use once_cell::sync::Lazy;

    pub struct InputParser;

    impl InputParser {
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
                "/exit" | "/quit" => Some(Command::Exit),
                _ => None,
            }
        }
    }

    pub enum Command<'a> {
        Help,
        ShowToday,
        Search(String),
        Settings,
        Clear,
        Exit,
    }
}

pub struct InputHandler {
    parser: InputParser,
    input_history: VecDeque<String>,
    history_position: Option<usize>,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            parser: InputParser,
            input_history: VecDeque::with_capacity(100),
            history_position: None,
        }
    }

    pub fn handle_input(&mut self, input: &str) -> InputResult {
        let trimmed = input.trim();

        if trimmed.is_empty() {
            return InputResult::Clear;
        }

        if let Some(command) = self.parser.parse_command(trimmed) {
            return self.execute_command(command);
        }

        if self.parser.is_json(trimmed) {
            return InputResult::Info("JSON input detected".to_string());
        }

        InputResult::Processing(trimmed.to_string())
    }

    fn execute_command(&self, command: parser::Command<'_>) -> InputResult {
        match command {
            parser::Command::Help => InputResult::ShowHelp,
            parser::Command::ShowToday => InputResult::Info("Showing today's events".to_string()),
            parser::Command::Search(query) => InputResult::Search(query),
            parser::Command::Settings => InputResult::OpenSettings,
            parser::Command::Clear => {
                InputResult::Clear
            }
            parser::Command::Exit => InputResult::Exit,
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
    Exit,
    Error(String),
}
