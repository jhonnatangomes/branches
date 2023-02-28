use std::{io, process::Command};

use chrono::{DateTime, FixedOffset};
use crossterm::{
    event::{read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::CrosstermBackend,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
    Terminal,
};

fn main() -> Result<(), io::Error> {
    let output = Command::new("git")
        .args(["branch", "-a", "-v"])
        .output()
        .unwrap();
    let str_output = String::from_utf8(output.stdout).unwrap();
    let mut hashes: Vec<CommitLog> = str_output
        .lines()
        .map(|l| &l[2..])
        .filter(|l| !l.starts_with("remotes/"))
        .map(get_last_commit_info)
        .collect();
    hashes.sort_by(|a, b| b.date.timestamp().cmp(&a.date.timestamp()));
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut list_state = ListState::default();
    list_state.select(Some(0));
    let item_contents: Vec<String> = hashes.iter().map(format_commit_info).collect();
    let mut selected_items: Vec<usize> = vec![];
    loop {
        terminal.draw(|f| {
            let size = f.size();
            let items: Vec<_> = item_contents
                .iter()
                .enumerate()
                .map(|(i, item)| {
                    if selected_items.contains(&i) {
                        return ListItem::new(item.clone()).style(Style::default().fg(Color::Red));
                    }
                    return ListItem::new(item.clone());
                })
                .collect();
            let highlight_color = if selected_items.contains(&list_state.selected().unwrap()) {
                Color::Rgb(255, 160, 137)
            } else {
                Color::Yellow
            };
            let list = List::new(items)
                .block(Block::default().title("List").borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .highlight_style(
                    Style::default()
                        .add_modifier(Modifier::ITALIC)
                        .fg(highlight_color),
                )
                .highlight_symbol(">>");
            f.render_stateful_widget(list, size, &mut list_state);
        })?;
        match read()? {
            Event::Key(k) => match k.code {
                KeyCode::Char('q') => break,
                KeyCode::Down | KeyCode::Char('j') => {
                    let new_index = (list_state.selected().unwrap() + 1) % item_contents.len();
                    list_state.select(Some(new_index))
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    let current_index = list_state.selected().unwrap();
                    let new_index = if current_index == 0 {
                        item_contents.len() - 1
                    } else {
                        current_index - 1
                    };
                    list_state.select(Some(new_index))
                }
                KeyCode::Char(' ') => {
                    let selected = list_state.selected().unwrap();
                    selected_items.push(selected);
                }
                _ => (),
            },
            _ => (),
        }
    }
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;
    Ok(())
}

// fn branches(line: &str) -> Vec<&str> {
//     line.split(" ").filter(|w| !w.is_empty()).collect()
// }
struct CommitLog {
    branch_name: String,
    hash_and_commit: String,
    author: String,
    date: DateTime<FixedOffset>,
}
fn get_last_commit_info(line: &str) -> CommitLog {
    let branch: Vec<_> = line.split(" ").filter(|w| !w.is_empty()).collect();
    let commit_hash = branch[1];
    let commit_log = String::from_utf8(
        Command::new("git")
            .args(["log", "-n", "1", commit_hash])
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap();
    let commit_log: Vec<_> = commit_log
        .lines()
        .filter(|l| l.starts_with("Author") || l.starts_with("Date"))
        .collect();
    CommitLog {
        branch_name: String::from(branch[0]),
        hash_and_commit: branch[1..].join(" "),
        author: String::from(commit_log[0]),
        date: DateTime::parse_from_str(commit_log[1], "Date:   %a %b %e %T %Y %z")
            .expect("Date parsing failed"),
    }
}

fn format_commit_info(commit_log: &CommitLog) -> String {
    format!(
        "{:<80}{}\r\n{:<80}{}\r\n{:<80}Date: {}",
        commit_log.branch_name,
        commit_log.hash_and_commit,
        "",
        commit_log.author,
        "",
        commit_log.date,
    )
}
