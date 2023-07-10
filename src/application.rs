use std::{env, error::Error, io};

use crossterm::{execute, terminal::enable_raw_mode, terminal::EnterAlternateScreen};
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::discord::gateway::Session;

pub struct Application {
    session: Session,
}

impl Application {
    pub fn new() -> Self {
        let token = env::var("DISCORDO_TOKEN").expect("missing authentication token");
        Self {
            session: Session::new(token),
        }
    }

    pub async fn run(&self) -> Result<(), Box<dyn Error>> {
        let mut stdout = io::stdout();
        enable_raw_mode()?;
        execute!(stdout, EnterAlternateScreen)?;

        let backend = CrosstermBackend::new(stdout);
        let _ = Terminal::new(backend);

        self.session.run().await
    }
}
