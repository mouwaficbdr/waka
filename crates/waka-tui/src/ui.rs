//! Rendering for the TUI application.
//!
//! This module contains all `ratatui` widget code. It is currently a stub
//! and will be populated in Task 2.7.

use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::App;

/// Renders the entire TUI screen.
pub fn render(f: &mut Frame, app: &App) {
    let size = f.area();

    // For now, just show a placeholder message.
    let title = format!("waka dashboard — view: {:?}", app.view);
    let block = Block::default()
        .title(title.as_str())
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan));

    let mut text_lines = vec![];
    text_lines.push(Line::from("Dashboard skeleton loaded."));
    text_lines.push(Line::from(""));
    if let Some(ref summary) = app.summary_today {
        let total = summary.data.first().map_or_else(
            || "No data".to_owned(),
            |d| format!("Total today: {:?}", d.grand_total.text),
        );
        text_lines.push(Line::from(total));
    } else if app.loading {
        text_lines.push(Line::from("Loading..."));
    } else if let Some(ref err) = app.error {
        text_lines.push(Line::from(Span::styled(
            format!("Error: {err}"),
            Style::default().fg(Color::Red),
        )));
    } else {
        text_lines.push(Line::from("No data yet."));
    }

    text_lines.push(Line::from(""));
    text_lines.push(Line::from("Press q to quit, ? for help."));

    let paragraph = Paragraph::new(text_lines).block(block);
    f.render_widget(paragraph, size);

    // Render help overlay if visible.
    if app.show_help {
        render_help_overlay(f, size);
    }
}

/// Renders the help overlay popup.
fn render_help_overlay(f: &mut Frame, area: Rect) {
    let block = Block::default()
        .title("Help")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Black).fg(Color::White));

    let help_text = vec![
        Line::from(Span::styled(
            "Keyboard Shortcuts",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("q / Esc      Quit"),
        Line::from("?            Toggle this help"),
        Line::from("Tab / 1-5    Switch view"),
        Line::from("r            Refresh"),
        Line::from("↑ / ↓        Navigate list"),
        Line::from("Enter        View details"),
        Line::from(""),
        Line::from("Press any key to close this help."),
    ];

    let paragraph = Paragraph::new(help_text).block(block);

    // Center the popup (40x12 rect).
    let popup_area = centered_rect(40, 12, area);
    f.render_widget(paragraph, popup_area);
}

/// Returns a centered `Rect` with the given percentage width and height.
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
