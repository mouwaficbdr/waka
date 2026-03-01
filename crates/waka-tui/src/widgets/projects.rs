//! Top Projects table widget.

#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss
)]

use ratatui::{
    layout::{Alignment, Constraint, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Cell, Row, Table},
    Frame,
};
use waka_api::SummaryResponse;

/// Renders the "Top Projects" table showing today's top projects.
///
/// Expected layout:
/// ```text
/// ┌─ Top Projects ────────────────────────────────────────────┐
/// │  my-saas         ████████████░░░░░░░░  3h 12m  48%        │
/// │  wakatime-cli    ███████░░░░░░░░░░░░░  2h 01m  30%        │
/// │  dotfiles        ███░░░░░░░░░░░░░░░░░    29m   7%         │
/// └───────────────────────────────────────────────────────────┘
/// ```
pub fn render_projects(frame: &mut Frame, area: Rect, summary: &SummaryResponse) {
    let block = Block::default()
        .title(" Top Projects ")
        .borders(Borders::ALL);

    let Some(today) = summary.data.first() else {
        let placeholder = ratatui::widgets::Paragraph::new("No data")
            .block(block)
            .alignment(Alignment::Center);
        frame.render_widget(placeholder, area);
        return;
    };

    // Sort projects by total_seconds descending, limit to 5
    let mut projects = today.projects.clone();
    projects.sort_by(|a, b| b.total_seconds.partial_cmp(&a.total_seconds).unwrap());
    let top5 = &projects[..projects.len().min(5)];

    if top5.is_empty() {
        let placeholder = ratatui::widgets::Paragraph::new("No projects today")
            .block(block)
            .alignment(Alignment::Center);
        frame.render_widget(placeholder, area);
        return;
    }

    let rows: Vec<Row> = top5
        .iter()
        .map(|proj| {
            let hours = (proj.total_seconds / 3600.0).floor() as u64;
            let minutes = ((proj.total_seconds % 3600.0) / 60.0).floor() as u64;
            let time_str = format!("{hours}h {minutes:02}m");

            // Build ASCII bar: 20 chars wide
            let bar_filled = ((proj.percent / 100.0) * 20.0).floor() as usize;
            let bar_empty = 20 - bar_filled;
            let bar = format!("{}{}", "█".repeat(bar_filled), "░".repeat(bar_empty));

            Row::new(vec![
                Cell::from(proj.name.clone()),
                Cell::from(bar).style(Style::default().fg(Color::Green)),
                Cell::from(time_str).style(Style::default().fg(Color::Cyan)),
                Cell::from(format!("{:.0}%", proj.percent))
                    .style(Style::default().fg(Color::Yellow)),
            ])
        })
        .collect();

    let widths = [
        Constraint::Percentage(30),
        Constraint::Percentage(40),
        Constraint::Percentage(15),
        Constraint::Percentage(15),
    ];

    let table = Table::new(rows, widths)
        .block(block)
        .style(Style::default());

    frame.render_widget(table, area);
}
