use std::{error::Error, io};

use crossterm::{execute, terminal::enable_raw_mode, terminal::EnterAlternateScreen};
use ratatui::{backend::CrosstermBackend, Terminal};

pub struct Application;

impl Application {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let mut stdout = io::stdout();
        enable_raw_mode()?;
        execute!(stdout, EnterAlternateScreen)?;

        let backend = CrosstermBackend::new(stdout);
        let _ = Terminal::new(backend);

        Ok(())
    }
}
