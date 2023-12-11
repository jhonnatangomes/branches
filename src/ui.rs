use std::io::Result;

use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::{style::Stylize, widgets::Paragraph};

use crate::App;

pub fn start_ui_loop(mut app: App) -> Result<()> {
    loop {
        app.draw(|frame| {
            let area = frame.size();
            frame.render_widget(
                Paragraph::new("Hello Ratatui! (press 'q' to quit)")
                    .white()
                    .on_blue(),
                area,
            );
        })?;
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }
    Ok(())
}
