use std::io::Result;

use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::{
    prelude::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    symbols,
    text::{Line, Span},
    widgets::{Block, Borders, LineGauge, List, ListItem, ListState, Paragraph},
};

use crate::{
    branches::{delete_branch, get_current_branches, Branch},
    App,
};

pub fn start_ui_loop(mut app: App) -> Result<()> {
    let branches = get_current_branches();
    let mut selected_branch = &branches[0];
    let mut selected_items = vec![];
    let mut progress = 0.0;
    let mut to_delete = false;
    loop {
        if to_delete {
            let initial_length = selected_items.len();
            if let Some(branch) = selected_items.pop() {
                delete_branch(branch);
                progress = 1.0 - (selected_items.len() as f64 / initial_length as f64);
            } else {
                break;
            }
        }
        app.draw(|frame| {
            let main_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(80),
                    Constraint::Percentage(10),
                    Constraint::Percentage(10),
                ])
                .split(frame.size());
            let items: Vec<_> = branches
                .iter()
                .enumerate()
                .map(|(i, branch)| {
                    ListItem::new(branch.to_string())
                        .style(item_style(&selected_items, &branches[i]))
                })
                .collect();
            let mut list_state = ListState::default()
                .with_selected(Some(selected_branch_index(&branches, selected_branch)));
            let list = List::new(items)
                .style(Style::default().fg(Color::White))
                .highlight_style(selected_branch_style(&selected_items, selected_branch))
                .highlight_symbol(">>");
            let text = vec![
                Line::from(Span::raw("Press j/k or ↑/↓ to navigate up and down\n")),
                Line::from(Span::raw("Press space to select/deselect a branch\n")),
                Line::from(Span::raw("Press enter to delete all selected branches\n")),
                Line::from(Span::raw("Press q to quit\n")),
            ];
            let paragraph = Paragraph::new(text);
            frame.render_stateful_widget(list, main_layout[0], &mut list_state);
            frame.render_widget(paragraph, main_layout[1]);
            if to_delete {
                let progress_bar = LineGauge::default()
                    .block(Block::default().borders(Borders::ALL).title("Progress"))
                    .gauge_style(
                        Style::default()
                            .fg(Color::White)
                            .bg(Color::Black)
                            .add_modifier(Modifier::BOLD),
                    )
                    .line_set(symbols::line::THICK)
                    .ratio(progress);
                frame.render_widget(progress_bar, main_layout[2]);
            }
        })?;
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && !to_delete {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('j') | KeyCode::Down => {
                            let selected_branch_index =
                                selected_branch_index(&branches, selected_branch);
                            let next_selected_branch_index =
                                (selected_branch_index + 1) % branches.len();
                            selected_branch = &branches[next_selected_branch_index];
                        }
                        KeyCode::Char('k') | KeyCode::Up => {
                            let selected_branch_index =
                                selected_branch_index(&branches, selected_branch);
                            let previous_selected_branch_index =
                                (selected_branch_index + branches.len() - 1) % branches.len();
                            selected_branch = &branches[previous_selected_branch_index];
                        }
                        KeyCode::Char(' ') => {
                            if selected_items.contains(&selected_branch) {
                                selected_items.retain(|i| i != selected_branch);
                            } else {
                                selected_items.push(selected_branch.clone());
                            }
                        }
                        KeyCode::Enter => {
                            to_delete = true;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    Ok(())
}

fn selected_branch_index(branches: &Vec<Branch>, selected_branch: &Branch) -> usize {
    branches
        .iter()
        .position(|b| b == selected_branch)
        .unwrap_or_default()
}

fn selected_branch_style(selected_items: &Vec<Branch>, selected_branch: &Branch) -> Style {
    let color = if selected_items.contains(selected_branch) {
        Color::Yellow
    } else {
        Color::Green
    };
    Style::default().add_modifier(Modifier::ITALIC).fg(color)
}

fn item_style(selected_items: &Vec<Branch>, branch: &Branch) -> Style {
    let color = if selected_items.contains(branch) {
        Color::Red
    } else {
        Color::White
    };
    Style::default().fg(color)
}
