use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::{CrosstermBackend, Terminal};
use std::io::{stdout, Result, Stdout};
use ui::start_ui_loop;

mod branches;
mod ui;
pub type App = Terminal<CrosstermBackend<Stdout>>;

fn main() -> Result<()> {
    let terminal = initialize()?;
    start_ui_loop(terminal)?;
    cleanup()?;
    Ok(())
}

fn initialize() -> Result<App> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;
    Ok(terminal)
}

fn cleanup() -> Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
