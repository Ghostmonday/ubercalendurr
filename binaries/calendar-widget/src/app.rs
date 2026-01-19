use std::sync::Arc;
use crate::AppState;

pub struct App {
    state: Arc<AppState>,
}

impl App {
    pub fn new(state: Arc<AppState>) -> Result<Self, std::io::Error> {
        Ok(Self { state })
    }

    pub fn run(&mut self) -> Result<(), std::io::Error> {
        println!("UberCalendurr Widget v0.1.0");
        println!("Type /help for available commands");
        println!();

        self.run_interactive()
    }

    fn run_interactive(&mut self) -> Result<(), std::io::Error> {
        use std::io::{self, Write};

        let stdin = io::stdin();
        let mut input = String::new();

        loop {
            print!("> ");
            io::stdout().flush()?;

            input.clear();
            let bytes_read = stdin.read_line(&mut input)?;

            if bytes_read == 0 {
                break;
            }

            let input = input.trim().to_string();

            if input.is_empty() {
                continue;
            }

            if input == "/exit" || input == "/quit" {
                break;
            }

            println!("Processing: {}", input);
        }

        Ok(())
    }
}
